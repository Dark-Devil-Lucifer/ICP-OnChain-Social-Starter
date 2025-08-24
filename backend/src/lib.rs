use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::time;
use ic_cdk::caller;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

type UserId = Principal;
type PostId = u64;
type Timestamp = u64;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Profile {
    pub username: String,
    pub avatar_url: String,
    pub created_at: Timestamp,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Comment {
    pub author: UserId,
    pub content: String,
    pub created_at: Timestamp,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Post {
    pub id: PostId,
    pub author: UserId,
    pub content: String,
    pub created_at: Timestamp,
    pub likes: BTreeSet<UserId>,
    pub comments: Vec<Comment>,
}

#[derive(Default, CandidType, Deserialize)]
struct State {
    next_post_id: PostId,
    profiles: BTreeMap<UserId, Profile>,
    posts: BTreeMap<PostId, Post>,
    user_posts: BTreeMap<UserId, Vec<PostId>>,
    following: BTreeMap<UserId, BTreeSet<UserId>>, // follower -> following set
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

// ------------- Helpers -------------
fn now() -> Timestamp { time() }

fn ensure_profile(u: &UserId) -> Result<(), String> {
    STATE.with(|s| {
        if s.borrow().profiles.contains_key(u) {
            Ok(())
        } else {
            Err("Profile not found. Please register first.".into())
        }
    })
}

// ------------- Methods -------------

#[update]
fn register(username: String, avatar_url: String) -> Result<Profile, String> {
    let me = caller();
    let p = Profile { username, avatar_url, created_at: now() };
    STATE.with(|s| {
        s.borrow_mut().profiles.insert(me, p.clone());
    });
    Ok(p)
}

#[query]
fn get_profile(user: UserId) -> Option<Profile> {
    STATE.with(|s| s.borrow().profiles.get(&user).cloned())
}

#[update]
fn create_post(content: String) -> Result<Post, String> {
    let me = caller();
    ensure_profile(&me)?;
    if content.trim().is_empty() { return Err("Content cannot be empty".into()); }
    STATE.with(|s| {
        let mut st = s.borrow_mut();
        st.next_post_id += 1;
        let id = st.next_post_id;
        let post = Post {
            id,
            author: me,
            content,
            created_at: now(),
            likes: BTreeSet::new(),
            comments: vec![],
        };
        st.user_posts.entry(me).or_default().push(id);
        st.posts.insert(id, post.clone());
        Ok(post)
    })
}

#[update]
fn edit_post(id: PostId, new_content: String) -> Result<Post, String> {
    let me = caller();
    STATE.with(|s| {
        let mut st = s.borrow_mut();
        let post = st.posts.get_mut(&id).ok_or("Post not found")?;
        if post.author != me { return Err("Only the author can edit".into()); }
        if new_content.trim().is_empty() { return Err("Content cannot be empty".into()); }
        post.content = new_content;
        Ok(post.clone())
    })
}

#[update]
fn delete_post(id: PostId) -> Result<(), String> {
    let me = caller();
    STATE.with(|s| {
        let mut st = s.borrow_mut();
        let post = st.posts.get(&id).ok_or("Post not found")?.clone();
        if post.author != me { return Err("Only the author can delete".into()); }
        st.posts.remove(&id);
        if let Some(list) = st.user_posts.get_mut(&me) {
            list.retain(|pid| pid != &id);
        }
        Ok(())
    })
}

#[update]
fn follow(user: UserId) -> Result<(), String> {
    let me = caller();
    if user == me { return Err("Cannot follow yourself".into()); }
    ensure_profile(&me)?;
    ensure_profile(&user)?;
    STATE.with(|s| {
        s.borrow_mut().following.entry(me).or_default().insert(user);
    });
    Ok(())
}

#[update]
fn unfollow(user: UserId) -> Result<(), String> {
    let me = caller();
    STATE.with(|s| {
        if let Some(set) = s.borrow_mut().following.get_mut(&me) {
            set.remove(&user);
        }
    });
    Ok(())
}

#[update]
fn like_post(id: PostId) -> Result<usize, String> {
    let me = caller();
    ensure_profile(&me)?;
    STATE.with(|s| {
        let mut st = s.borrow_mut();
        let post = st.posts.get_mut(&id).ok_or("Post not found")?;
        post.likes.insert(me);
        Ok(post.likes.len())
    })
}

#[update]
fn unlike_post(id: PostId) -> Result<usize, String> {
    let me = caller();
    ensure_profile(&me)?;
    STATE.with(|s| {
        let mut st = s.borrow_mut();
        let post = st.posts.get_mut(&id).ok_or("Post not found")?;
        post.likes.remove(&me);
        Ok(post.likes.len())
    })
}

#[update]
fn comment_post(id: PostId, content: String) -> Result<Post, String> {
    let me = caller();
    ensure_profile(&me)?;
    if content.trim().is_empty() { return Err("Comment cannot be empty".into()); }
    STATE.with(|s| {
        let mut st = s.borrow_mut();
        let post = st.posts.get_mut(&id).ok_or("Post not found")?;
        post.comments.push(Comment { author: me, content, created_at: now() });
        Ok(post.clone())
    })
}

#[query]
fn get_post(id: PostId) -> Option<Post> {
    STATE.with(|s| s.borrow().posts.get(&id).cloned())
}

#[query]
fn get_user_posts(user: UserId) -> Vec<Post> {
    STATE.with(|s| {
        let st = s.borrow();
        st.user_posts.get(&user).cloned().unwrap_or_default()
            .into_iter()
            .filter_map(|pid| st.posts.get(&pid).cloned())
            .collect()
    })
}

#[query]
fn get_following(user: UserId) -> Vec<UserId> {
    STATE.with(|s| s.borrow().following.get(&user).map(|set| set.iter().cloned().collect()).unwrap_or_default())
}

#[query]
fn get_feed_for(user: UserId, offset: usize, limit: usize) -> Vec<Post> {
    STATE.with(|s| {
        let st = s.borrow();
        let mut authors: BTreeSet<UserId> = BTreeSet::new();
        authors.insert(user);
        if let Some(f) = st.following.get(&user) {
            for u in f.iter() { authors.insert(*u); }
        }
        let mut posts: Vec<Post> = st.posts.values()
            .filter(|p| authors.contains(&p.author))
            .cloned()
            .collect();
        // sort by created_at desc then id desc
        posts.sort_by(|a,b| b.created_at.cmp(&a.created_at).then(b.id.cmp(&a.id)));
        let end = (offset + limit).min(posts.len());
        if offset >= posts.len() { vec![] } else { posts[offset..end].to_vec() }
    })
}

#[query]
fn get_my_feed(offset: usize, limit: usize) -> Vec<Post> {
    let me = caller();
    get_feed_for(me, offset, limit)
}

// ----------------- Upgrades -----------------

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|s| {
        let st = s.borrow();
        ic_cdk::storage::stable_save((st,)).expect("failed to save state");
    });
}

#[post_upgrade]
fn post_upgrade() {
    let (st,): (State,) = ic_cdk::storage::stable_restore().unwrap_or_default();
    STATE.with(|s| *s.borrow_mut() = st);
}

// ----------------- Demo Data -----------------

#[init]
fn init() {
    // Demo principals (publicly known examples). Replace or extend as needed.
    // These are constant text principals used for preloading data.
    let users = vec![
        ("w7x7r-cok77-xa", "alice", "https://i.pravatar.cc/150?img=1"),
        ("qjdve-lqaaa-aaaaa-aaaeq-cai", "bob", "https://i.pravatar.cc/150?img=2"),
        ("rdmx6-jaaaa-aaaaa-aaadq-cai", "charlie", "https://i.pravatar.cc/150?img=3"),
        ("rwlgt-iiaaa-aaaaa-aaaaa-cai", "diana", "https://i.pravatar.cc/150?img=4"),
        ("renrk-eyaaa-aaaaa-aaada-cai", "eve", "https://i.pravatar.cc/150?img=5"),
    ];

    STATE.with(|s| {
        let mut st = s.borrow_mut();
        for (p, name, avatar) in users.iter() {
            let pid = Principal::from_text(p.to_string()).unwrap_or(Principal::anonymous());
            st.profiles.insert(pid, Profile {
                username: name.to_string(),
                avatar_url: avatar.to_string(),
                created_at: now(),
            });
            st.following.entry(pid).or_default(); // ensure map entry
        }

        // Follows
        let alice = Principal::from_text("w7x7r-cok77-xa").unwrap();
        let bob = Principal::from_text("qjdve-lqaaa-aaaaa-aaaeq-cai").unwrap();
        let charlie = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
        let diana = Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap();
        let eve = Principal::from_text("renrk-eyaaa-aaaaa-aaada-cai").unwrap();

        st.following.entry(alice).or_default().extend([bob, charlie]);
        st.following.entry(bob).or_default().extend([alice, diana]);
        st.following.entry(charlie).or_default().extend([alice, eve]);
        st.following.entry(diana).or_default().extend([alice, bob, eve]);
        st.following.entry(eve).or_default().extend([charlie]);

        // Posts (2–3 each)
        let mut add_post = |author: UserId, content: &str| {
            st.next_post_id += 1;
            let id = st.next_post_id;
            let post = Post {
                id,
                author,
                content: content.into(),
                created_at: now(),
                likes: BTreeSet::new(),
                comments: vec![],
            };
            st.user_posts.entry(author).or_default().push(id);
            st.posts.insert(id, post);
            id
        };

        let a1 = add_post(alice, "Hello ICP! First post from Alice.");
        let a2 = add_post(alice, "Loving fully on-chain social!");

        let b1 = add_post(bob, "Bob here. Rust canisters FTW.");
        let b2 = add_post(bob, "Working on the feed today.");
        let b3 = add_post(bob, "Any tips for stable memory?");

        let c1 = add_post(charlie, "Query calls make feeds snappy.");
        let c2 = add_post(charlie, "Subscriptions or polling? I prefer light polling.");

        let d1 = add_post(diana, "Frontend wired up with Vite.");
        let d2 = add_post(diana, "Internet Identity login is smooth!");

        let e1 = add_post(eve, "Eve testing comments & likes.");
        let e2 = add_post(eve, "Decentralized storage > centralized servers.");

        // Likes
        let mut like = |pid: PostId, who: UserId| {
            if let Some(p) = st.posts.get_mut(&pid) {
                p.likes.insert(who);
            }
        };

        like(a1, bob); like(a1, charlie);
        like(b1, alice); like(b1, diana);
        like(c1, alice); like(c1, bob); like(c1, eve);
        like(d2, alice); like(d2, bob); like(d2, charlie);
        like(e1, alice); like(e1, diana);

        // Comments
        let mut comment = |pid: PostId, who: UserId, text: &str| {
            if let Some(p) = st.posts.get_mut(&pid) {
                p.comments.push(Comment { author: who, content: text.into(), created_at: now() });
            }
        };

        comment(a1, bob, "Welcome Alice!");
        comment(b1, diana, "Absolutely!");
        comment(c1, eve, "Agreed!");
        comment(d2, alice, "Thanks Diana!");
        comment(e1, alice, "Looks good.");
    });
}