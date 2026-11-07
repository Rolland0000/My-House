import type { FormEvent } from "react";
import { Search } from "lucide-react";
import { Input } from "../../../shared/components";
import { useDebouncedUrlDraft } from "../../../shared/hooks/useDebouncedUrlDraft";
import { MAX_SEARCH_CHARS } from "../adminUsersCriteria";

interface AdminUsersSearchProps {
  value: string;
  /** `settled` is true when the text applied itself after a typing pause, false on Enter. */
  onSearch: (text: string, settled: boolean) => void;
}

function AdminUsersSearch({ value, onSearch }: AdminUsersSearchProps) {
  const [draft, setDraft] = useDebouncedUrlDraft(value, (text) => onSearch(text.trim(), true));

  function handleSubmit(event: FormEvent) {
    event.preventDefault();
    onSearch(draft.trim(), false);
  }

  return (
    <form role="search" onSubmit={handleSubmit} className="min-w-60 flex-1">
      <Input
        type="search"
        aria-label="Search by email or name"
        placeholder="Search by email or name"
        maxLength={MAX_SEARCH_CHARS}
        leftIcon={<Search className="size-4" aria-hidden="true" />}
        value={draft}
        onChange={(event) => setDraft(event.target.value)}
      />
    </form>
  );
}

export { AdminUsersSearch };
