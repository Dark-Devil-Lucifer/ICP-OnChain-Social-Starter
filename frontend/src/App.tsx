import React from "react";

const users = [
  {
    name: "Alice Johnson",
    bio: "Blockchain enthusiast & content creator",
    posts: 12,
    avatar: "https://i.pravatar.cc/100?img=1",
  },
  {
    name: "Bob Smith",
    bio: "Web3 developer exploring decentralized apps",
    posts: 8,
    avatar: "https://i.pravatar.cc/100?img=2",
  },
  {
    name: "Charlie Lee",
    bio: "Digital artist sharing NFT collections",
    posts: 15,
    avatar: "https://i.pravatar.cc/100?img=3",
  },
  {
    name: "Diana Patel",
    bio: "Tech writer covering decentralized networks",
    posts: 20,
    avatar: "https://i.pravatar.cc/100?img=4",
  },
  {
    name: "Ethan Wu",
    bio: "Crypto trader with interest in DeFi",
    posts: 5,
    avatar: "https://i.pravatar.cc/100?img=5",
  },
];

export default function App() {
  return (
    <div className="min-h-screen bg-gradient-to-r from-gray-900 via-black to-gray-800 text-white">
      {/* Navbar */}
      <nav className="flex justify-between items-center px-6 py-4 bg-gray-900 shadow-md">
        <h1 className="text-2xl font-bold">OnChain Social</h1>
        <div className="space-x-6">
          <a href="#" className="hover:text-blue-400">Home</a>
          <a href="#" className="hover:text-blue-400">About</a>
          <button className="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg">Connect Wallet</button>
        </div>
      </nav>

      {/* Hero Section */}
      <section className="text-center py-16 px-4">
        <h2 className="text-4xl font-bold mb-4">OnChain Social Network</h2>
        <p className="text-gray-300 max-w-2xl mx-auto">
          A fully decentralized social media platform built on the ICP blockchain, where content, connections, and data are entirely on-chain.
        </p>
        <button className="mt-6 px-6 py-3 bg-blue-600 hover:bg-blue-700 rounded-lg text-lg">
          Get Started
        </button>
      </section>

      {/* Users */}
      <section className="px-6 py-12">
        <h3 className="text-2xl font-semibold mb-6 text-center">Meet Our Users</h3>
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-6">
          {users.map((user, idx) => (
            <div key={idx} className="bg-gray-800 p-6 rounded-2xl shadow-lg hover:scale-105 transition">
              <img
                src={user.avatar}
                alt={user.name}
                className="w-20 h-20 rounded-full mx-auto mb-4 border-2 border-blue-500"
              />
              <h4 className="text-xl font-bold text-center">{user.name}</h4>
              <p className="text-gray-400 text-center mt-2">{user.bio}</p>
              <p className="text-gray-300 text-center mt-2">Posts: {user.posts}</p>
            </div>
          ))}
        </div>
      </section>
    </div>
  );
}

