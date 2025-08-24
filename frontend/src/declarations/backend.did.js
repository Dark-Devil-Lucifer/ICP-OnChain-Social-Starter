export const idlFactory = ({ IDL }) => {
  const Timestamp = IDL.Nat64;
  const Profile = IDL.Record({
    'username': IDL.Text,
    'avatar_url': IDL.Text,
    'created_at': Timestamp,
  });
  const Comment = IDL.Record({
    'author': IDL.Principal,
    'content': IDL.Text,
    'created_at': Timestamp,
  });
  const PostId = IDL.Nat64;
  const Post = IDL.Record({
    'id': PostId,
    'author': IDL.Principal,
    'content': IDL.Text,
    'created_at': Timestamp,
    'likes': IDL.Vec(IDL.Principal),
    'comments': IDL.Vec(Comment),
  });
  const ResultProfile = IDL.Variant({ 'ok': Profile, 'err': IDL.Text });
  const ResultPost = IDL.Variant({ 'ok': Post, 'err': IDL.Text });
  const ResultVoid = IDL.Variant({ 'ok': IDL.Null, 'err': IDL.Text });
  const ResultNat = IDL.Variant({ 'ok': IDL.Nat, 'err': IDL.Text });

  return IDL.Service({
    'register': IDL.Func([IDL.Text, IDL.Text], [ResultProfile], []),
    'get_profile': IDL.Func([IDL.Principal], [IDL.Opt(Profile)], ['query']),
    'create_post': IDL.Func([IDL.Text], [ResultPost], []),
    'edit_post': IDL.Func([PostId, IDL.Text], [ResultPost], []),
    'delete_post': IDL.Func([PostId], [ResultVoid], []),
    'follow': IDL.Func([IDL.Principal], [ResultVoid], []),
    'unfollow': IDL.Func([IDL.Principal], [ResultVoid], []),
    'like_post': IDL.Func([PostId], [ResultNat], []),
    'unlike_post': IDL.Func([PostId], [ResultNat], []),
    'comment_post': IDL.Func([PostId, IDL.Text], [ResultPost], []),
    'get_post': IDL.Func([PostId], [IDL.Opt(Post)], ['query']),
    'get_user_posts': IDL.Func([IDL.Principal], [IDL.Vec(Post)], ['query']),
    'get_following': IDL.Func([IDL.Principal], [IDL.Vec(IDL.Principal)], ['query']),
    'get_feed_for': IDL.Func([IDL.Principal, IDL.Nat, IDL.Nat], [IDL.Vec(Post)], ['query']),
    'get_my_feed': IDL.Func([IDL.Nat, IDL.Nat], [IDL.Vec(Post)], ['query']),
  });
};

export const init = ({ IDL }) => {
  return [];
};