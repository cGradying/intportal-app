// The app's one piece of shared state: the snapshot from the core, today's
// reading of it, and sync status. Screens read `app`; only this file writes.
import { api, moment, type Snapshot, type Today } from "./api";

export type Tab = "today" | "schedule" | "grades" | "you";
export type SyncState = "idle" | "syncing" | "ok" | "failed";

export const app = $state({
  snapshot: null as Snapshot | null,
  today: null as Today | null,
  tab: "today" as Tab,
  sync: "idle" as SyncState,
  syncError: null as string | null,
  /** Bumps on every successful sync, so moments can react to it. */
  syncCount: 0,
  room: (localStorage.getItem("room") as string) || "night",
});

export async function refreshToday() {
  if (!app.snapshot) return;
  try {
    app.today = await api.today(moment(), app.sync === "failed" ? app.syncError : null);
  } catch {
    // The glance keeps its last line.
  }
}

export async function adopt(snap: Snapshot) {
  app.snapshot = snap;
  await refreshToday();
}

export async function syncNow() {
  if (app.sync === "syncing") return;
  app.sync = "syncing";
  try {
    await adopt(await api.sync());
    app.sync = "ok";
    app.syncError = null;
    app.syncCount++;
  } catch (e) {
    app.sync = "failed";
    app.syncError = String(e).includes("reach") ? "Couldn't reach SIS" : String(e);
    await refreshToday();
  }
}

export function setRoom(room: string) {
  app.room = room;
  document.documentElement.dataset.room = room;
  try {
    localStorage.setItem("room", room);
  } catch {
    // Private storage off: the room lasts this session only.
  }
}

let ticker: ReturnType<typeof setInterval> | undefined;

/** Re-read today on the minute boundary, so countdowns stay honest. */
export function startClock() {
  clearInterval(ticker);
  const kick = () => {
    refreshToday();
    ticker = setInterval(refreshToday, 60_000);
  };
  setTimeout(kick, (60 - new Date().getSeconds()) * 1000);
}
