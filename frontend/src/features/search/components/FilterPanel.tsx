import { Fragment, useEffect, useEffectEvent, useId, useState } from "react";
import { ChevronDown } from "lucide-react";
import { Button, Checkbox, Input } from "../../../shared/components";
import {
  TEXT_FILTER_DEBOUNCE_MS,
  useDebouncedValue,
} from "../../../shared/hooks/useDebouncedValue";
import { cn } from "../../../shared/utils/cn";
import {
  AVAILABILITY_BADGE,
  typeLabels,
  type ListingStatus,
  type ListingType,
} from "../../listings";
import { useUrlDraft } from "../hooks/useUrlDraft";
import { PriceRangeFilter } from "./PriceRangeFilter";
import type { SearchCriteria } from "../searchCriteria";

type FilterPatch = Partial<Omit<SearchCriteria, "q" | "page">>;

interface FilterPanelProps {
  criteria: SearchCriteria;
  /** A new value remounts the text and price fields, dropping their unsent drafts. */
  draftResetKey: number;
  onChange: (patch: FilterPatch) => void;
  onClearAll: () => void;
}

const typeEntries = Object.entries(typeLabels) as [ListingType, string][];
const statusEntries = Object.entries(AVAILABILITY_BADGE) as [ListingStatus, { label: string }][];

function toggle<T>(values: T[], value: T, checked: boolean): T[] {
  return checked ? [...values, value] : values.filter((item) => item !== value);
}

interface TextFilterProps {
  label: string;
  /** Value currently in the URL. */
  value: string;
  onCommit: (value: string) => void;
}

function TextFilter({ label, value, onCommit }: TextFilterProps) {
  const inputId = useId();
  const [draft, setDraft] = useUrlDraft(value);

  // An emptied field applies at once.
  const debouncedDraft = useDebouncedValue(draft, draft ? TEXT_FILTER_DEBOUNCE_MS : 0);

  // Skips a value already in the URL, which would otherwise reset the page on mount or after a resync.
  const commit = useEffectEvent((next: string) => {
    if (next.trim() !== value) onCommit(next);
  });
  // Deferred past React's commit phase, where the navigation's flushSync is not allowed.
  useEffect(() => queueMicrotask(() => commit(debouncedDraft)), [debouncedDraft]);

  return (
    <div className="flex flex-col gap-1.5">
      <label htmlFor={inputId} className="text-sm font-semibold text-text">
        {label}
      </label>
      <Input id={inputId} value={draft} onChange={(event) => setDraft(event.target.value)} />
    </div>
  );
}

function FilterPanel({ criteria, draftResetKey, onChange, onClearAll }: FilterPanelProps) {
  const typeToggleId = useId();
  const typeOptionsId = useId();
  const [isTypeOpen, setIsTypeOpen] = useState(true);

  return (
    <div className="flex flex-col gap-6 rounded-sm border border-border bg-surface p-5">
      <section>
        <h2>
          <button
            id={typeToggleId}
            type="button"
            aria-expanded={isTypeOpen}
            aria-controls={typeOptionsId}
            onClick={() => setIsTypeOpen((open) => !open)}
            className="flex w-full items-center justify-between text-sm font-semibold text-text focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus"
          >
            Type
            <ChevronDown
              aria-hidden="true"
              className={cn("size-4 text-ink-500", isTypeOpen && "rotate-180")}
            />
          </button>
        </h2>
        <div
          id={typeOptionsId}
          role="group"
          aria-labelledby={typeToggleId}
          hidden={!isTypeOpen}
          className="mt-3 flex flex-col gap-2.5"
        >
          {typeEntries.map(([type, label]) => (
            <Checkbox
              key={type}
              label={label}
              checked={criteria.types.includes(type)}
              onChange={(event) =>
                onChange({ types: toggle(criteria.types, type, event.target.checked) })
              }
            />
          ))}
        </div>
      </section>

      <PriceRangeFilter
        // Sibling of the text fields' Fragment, so it needs a key of its own.
        key={`price-${draftResetKey}`}
        priceMin={criteria.priceMin}
        priceMax={criteria.priceMax}
        onCommit={onChange}
      />

      <fieldset className="flex flex-col gap-2.5">
        <legend className="mb-3 text-sm font-semibold text-text">Status</legend>
        {statusEntries.map(([status, { label }]) => (
          <Checkbox
            key={status}
            label={label}
            checked={criteria.statuses.includes(status)}
            onChange={(event) =>
              onChange({ statuses: toggle(criteria.statuses, status, event.target.checked) })
            }
          />
        ))}
      </fieldset>

      <Fragment key={draftResetKey}>
        <TextFilter label="City" value={criteria.city} onCommit={(city) => onChange({ city })} />
        <TextFilter
          label="Neighborhood"
          value={criteria.neighborhood}
          onCommit={(neighborhood) => onChange({ neighborhood })}
        />
      </Fragment>

      <Button variant="secondary" size="sm" onClick={onClearAll}>
        Clear all
      </Button>
    </div>
  );
}

export { FilterPanel };
export type { FilterPatch };
