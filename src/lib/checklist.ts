import type { ChecklistFacts } from './bindings';

export type ChecklistItem =
  | { kind: 'moonstoneChest'; biome: string | null }
  | { kind: 'dreamSnapSubmission' }
  | { kind: 'dreamSnapVoting' };

export function liveChecklist(facts: ChecklistFacts): ChecklistItem[] {
  const items: ChecklistItem[] = facts.moonstoneChestBiomes.map((biome) => ({
    kind: 'moonstoneChest',
    biome
  }));
  if (facts.dreamSnaps?.submitNeeded) items.push({ kind: 'dreamSnapSubmission' });
  if (facts.dreamSnaps?.voteNeeded) items.push({ kind: 'dreamSnapVoting' });
  return items;
}

export function checklistLabel(item: ChecklistItem): string {
  switch (item.kind) {
    case 'moonstoneChest':
      return item.biome ? `Daily Moonstone Chest (${item.biome})` : 'Daily Moonstone Chest';
    case 'dreamSnapSubmission':
      return 'DreamSnaps: Submit a photo';
    case 'dreamSnapVoting':
      return 'DreamSnaps: Vote';
  }
}
