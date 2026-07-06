import type { Section, Snapshot } from './bindings';
import { snapshot } from './snapshot.svelte';
import { clock } from './clock.svelte';
import { liveCritter, liveVillager } from './time';

function liveSection<T, L>(
  pick: (snap: Snapshot) => Section<T[]>,
  toLive: (item: T, nowSecs: number, tzOffset: number) => L
) {
  const section = $derived(snapshot.current === null ? null : pick(snapshot.current));
  const data = $derived.by(() => {
    const snap = snapshot.current;
    if (snap === null) return [];
    const section = pick(snap);
    if (section.status !== 'ok') return [];
    return section.data.map((item) => toLive(item, clock.nowSecs, snap.tzOffset));
  });

  return {
    get loading() {
      return snapshot.current === null;
    },
    get error() {
      return section?.status === 'error' ? section.error : '';
    },
    get data() {
      return data;
    }
  };
}

export const villagers = liveSection((snap) => snap.villagers, liveVillager);
export const critters = liveSection((snap) => snap.critters, liveCritter);
