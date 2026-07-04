import type { ChecklistFacts } from './bindings';
import { clock } from './clock.svelte';
import { formatDuration } from './utils';

export type ChecklistItem =
  | { kind: 'moonstoneChest'; biome: string | null }
  | { kind: 'rift'; biome: string | null }
  | { kind: 'upcomingRift'; biome: string | null; unixSeconds: number }
  | { kind: 'dreamSnapSubmission' }
  | { kind: 'dreamSnapVoting' };

export function liveChecklist(facts: ChecklistFacts): ChecklistItem[] {
  const items: ChecklistItem[] = facts.moonstoneChestBiomes.map((biome) => ({
    kind: 'moonstoneChest',
    biome
  }));
  for (const biome of facts.riftBiomes) items.push({ kind: 'rift', biome });
  for (const rift of facts.upcomingRifts) {
    if (rift.spawnSecs > clock.nowSecs) {
      items.push({ kind: 'upcomingRift', biome: rift.biome, unixSeconds: rift.spawnSecs });
    }
  }
  if (facts.dreamSnaps?.submitNeeded) items.push({ kind: 'dreamSnapSubmission' });
  if (facts.dreamSnaps?.voteNeeded) items.push({ kind: 'dreamSnapVoting' });
  return items;
}

export function checklistLabel(item: ChecklistItem): string {
  switch (item.kind) {
    case 'moonstoneChest':
      return item.biome ? `Daily Moonstone Chest (${item.biome})` : 'Daily Moonstone Chest';
    case 'rift':
      return item.biome ? `Time Rift (${item.biome})` : 'Time Rift';
    case 'upcomingRift': {
      const label = `Upcoming Time Rift - ${formatDuration(item.unixSeconds - clock.nowSecs)}`;
      return item.biome ? `${label} (${item.biome})` : label;
    }
    case 'dreamSnapSubmission':
      return 'DreamSnaps: Submit a photo';
    case 'dreamSnapVoting':
      return 'DreamSnaps: Vote';
  }
}
