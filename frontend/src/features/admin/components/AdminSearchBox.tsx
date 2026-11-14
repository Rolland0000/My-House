import type { FormEvent } from "react";
import { Search } from "lucide-react";
import { Input } from "../../../shared/components";
import { useDebouncedUrlDraft } from "../../../shared/hooks/useDebouncedUrlDraft";
import { MAX_SEARCH_CHARS } from "../adminUsersCriteria";

interface AdminSearchBoxProps {
  value: string;
  /** What the box searches, e.g. "email or name"; fills the label and the placeholder. */
  target: string;
  /** `settled` is true when the text applied itself after a typing pause, false on Enter. */
  onSearch: (text: string, settled: boolean) => void;
}

function AdminSearchBox({ value, target, onSearch }: AdminSearchBoxProps) {
  const [draft, setDraft] = useDebouncedUrlDraft(value, (text) => onSearch(text.trim(), true));

  function handleSubmit(event: FormEvent) {
    event.preventDefault();
    onSearch(draft.trim(), false);
  }

  return (
    <form role="search" onSubmit={handleSubmit} className="min-w-60 flex-1">
      <Input
        type="search"
        aria-label={`Search by ${target}`}
        placeholder={`Search by ${target}`}
        maxLength={MAX_SEARCH_CHARS}
        leftIcon={<Search className="size-4" aria-hidden="true" />}
        value={draft}
        onChange={(event) => setDraft(event.target.value)}
      />
    </form>
  );
}

export { AdminSearchBox };
