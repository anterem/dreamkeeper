import type { ChecklistFacts } from './bindings';

export type ChecklistItem =
  | { kind: 'moonstoneChest'; biome: string | null }
  | { kind: 'rift'; biome: string | null }
  | { kind: 'dreamSnapSubmission' }
  | { kind: 'dreamSnapVoting' }
  | { kind: 'scroogeStore'; location: string | null; count: number };

export function liveChecklist(facts: ChecklistFacts): ChecklistItem[] {
  const items: ChecklistItem[] = facts.moonstoneChestBiomes.map((biome) => ({
    kind: 'moonstoneChest',
    biome
  }));
  for (const biome of facts.riftBiomes) items.push({ kind: 'rift', biome });
  if (facts.dreamSnaps?.submitNeeded) items.push({ kind: 'dreamSnapSubmission' });
  if (facts.dreamSnaps?.voteNeeded) items.push({ kind: 'dreamSnapVoting' });
  for (const store of facts.scroogeStores)
    items.push({ kind: 'scroogeStore', location: store.location, count: store.count });
  return items;
}

export function checklistLabel(item: ChecklistItem): string {
  switch (item.kind) {
    case 'moonstoneChest':
      return item.biome ? `Daily Moonstone Chest (${item.biome})` : 'Daily Moonstone Chest';
    case 'rift':
      return item.biome ? `Time Rift (${item.biome})` : 'Time Rift';
    case 'dreamSnapSubmission':
      return 'DreamSnaps: Submit a photo';
    case 'dreamSnapVoting':
      return 'DreamSnaps: Vote';
    case 'scroogeStore': {
      const label = `Scrooge's Store: ${item.count} new item${item.count === 1 ? '' : 's'}`;
      return item.location ? `${label} (${item.location})` : label;
    }
  }
}
