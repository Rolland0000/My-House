import { X } from "lucide-react";

interface OwnerFilterChipProps {
  label: string;
  onRemove: () => void;
}

function OwnerFilterChip({ label, onRemove }: OwnerFilterChipProps) {
  return (
    <div className="inline-flex w-fit items-center gap-1 rounded-sm border border-border bg-surface py-1 pr-1 pl-3 text-sm font-semibold text-ink-900">
      {label}
      <button
        type="button"
        aria-label={`Remove filter: ${label}`}
        onClick={onRemove}
        className="flex size-6 items-center justify-center rounded-sm text-ink-500 hover:text-ink-900 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus"
      >
        <X className="size-4" aria-hidden="true" />
      </button>
    </div>
  );
}

export { OwnerFilterChip };
