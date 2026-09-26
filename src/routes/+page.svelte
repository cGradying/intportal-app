<script lang="ts">
  import "$lib/app.css";
  import { onMount } from "svelte";
  import Landing from "$lib/Landing.svelte";
  import Island from "$lib/Island.svelte";
  import TabBar from "$lib/TabBar.svelte";
  import Today from "$lib/screens/Today.svelte";
  import Schedule from "$lib/screens/Schedule.svelte";
  import Grades from "$lib/screens/Grades.svelte";
  import You from "$lib/screens/You.svelte";
  import { api } from "$lib/api";
  import { app, adopt, setRoom, startClock } from "$lib/state.svelte";

  let ready = $state(false);
  let signedIn = $state(false);
  let inside = $state(false);

  onMount(async () => {
    setRoom(app.room);
    const [login, snap] = await Promise.all([api.hasLogin().catch(() => false), api.snapshot().catch(() => null)]);
    signedIn = login;
    if (snap) await adopt(snap);
    startClock();
    ready = true;
  });

  // Each tab opens at its top, not wherever the last one was scrolled.
  $effect(() => {
    void app.tab;
    window.scrollTo(0, 0);
  });

  function enter() {
    inside = true;
  }

  function signedOut() {
    app.snapshot = null;
    app.today = null;
    app.tab = "today";
    signedIn = false;
    inside = false;
  }
</script>

{#if ready && !inside}
  <Landing {signedIn} onenter={enter} />
{/if}

{#if inside}
  <div class="shell dots">
    <div class="topfade" aria-hidden="true"></div>
    <Island />
    <main>
      {#key app.tab}
        <div class="page">
          {#if app.tab === "today"}<Today />
          {:else if app.tab === "schedule"}<Schedule />
          {:else if app.tab === "grades"}<Grades />
          {:else}<You onsignout={signedOut} />{/if}
        </div>
      {/key}
    </main>
    <TabBar />
  </div>
{/if}

<style>
  .shell {
    min-height: 100vh;
  }
  /* No fill mode: a lingering filter or transform would turn these into the
     containing block for the fixed island, tab bar and sheets. */
  .topfade {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 25;
    height: calc(var(--safe-top) + 52px);
    background: linear-gradient(var(--ground) 55%, transparent);
    pointer-events: none;
  }
  main {
    animation: arrive 0.65s var(--ease) backwards;
    padding: calc(var(--safe-top) + 58px) var(--gutter) calc(var(--safe-bottom) + 92px);
  }
  @keyframes arrive {
    from { filter: brightness(2.2) saturate(0.4); }
  }
  .page {
    animation: in 0.3s var(--ease) backwards;
  }
  @keyframes in {
    from { opacity: 0; transform: translateY(6px); }
  }
</style>
