import type { ChecklistItem } from './bindings';

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
