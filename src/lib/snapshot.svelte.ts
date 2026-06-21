import { commands, events, type Snapshot } from './bindings';

function createSnapshot() {
  let current = $state<Snapshot | null>(null);
  let listening = false;

  async function refresh() {
    const res = await commands.getSnapshot();
    if (res.status === 'ok') current = res.data;
  }

  async function init() {
    if (listening) return;
    listening = true;
    await events.saveChanged.listen((e) => {
      current = e.payload;
    });
    await refresh();
  }

  return {
    get current() {
      return current;
    },
    init,
    refresh
  };
}

export const snapshot = createSnapshot();
