import { HttpAgent, Actor } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import { idlFactory } from "./declarations/backend.did.js";

// Try multiple sources to find the backend canister id
function detectBackendCanisterId(): string {
  const envId = import.meta.env.VITE_BACKEND_CANISTER_ID as string | undefined;
  if (envId && envId.length > 0) return envId;

  const w = globalThis as any;
  if (w.__CANISTER_IDS__?.backend?.local) return w.__CANISTER_IDS__.backend.local;
  if (w.__CANISTER_IDS__?.backend?.ic) return w.__CANISTER_IDS__.backend.ic;
  return ""; // let caller warn the user
}

export type BackendActor = ReturnType<typeof createActor>;

export async function createActor() {
  const canisterId = detectBackendCanisterId();
  const authClient = await AuthClient.create();
  const isAuthenticated = await authClient.isAuthenticated();
  const identity = isAuthenticated ? authClient.getIdentity() : undefined;

  const agent = new HttpAgent({ identity });
  // In local dev, fetch root key for certificate validation
  try { await agent.fetchRootKey(); } catch {}

  const actor = Actor.createActor(idlFactory, {
    agent, canisterId
  });

  return { actor, authClient, canisterId };
}

export async function login(authClient: AuthClient) {
  await authClient.login({
    identityProvider: "https://identity.ic0.app/#authorize",
    onSuccess: () => location.reload()
  });
}

export async function logout(authClient: AuthClient) {
  await authClient.logout();
  location.reload();
}