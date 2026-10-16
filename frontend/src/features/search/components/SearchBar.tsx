import { Search } from "lucide-react";
import { Button, Input } from "../../../shared/components";
import { useUrlDraft } from "../hooks/useUrlDraft";
import { MAX_KEYWORD_CHARS } from "../searchCriteria";

interface SearchBarProps {
  /** Keyword currently in the URL. */
  value: string;
  onSubmit: (keyword: string) => void;
}

function SearchBar({ value, onSubmit }: SearchBarProps) {
  const [draft, setDraft] = useUrlDraft(value);

  return (
    <form
      role="search"
      className="flex gap-2"
      onSubmit={(event) => {
        event.preventDefault();
        onSubmit(draft.trim());
      }}
    >
      <div className="flex-1">
        <Input
          type="search"
          value={draft}
          onChange={(event) => setDraft(event.target.value)}
          maxLength={MAX_KEYWORD_CHARS}
          placeholder="Title, city, neighborhood, owner…"
          aria-label="Search keywords"
          leftIcon={<Search className="size-4" aria-hidden="true" />}
        />
      </div>
      <Button type="submit" size="sm" className="flex-none px-5">
        Search
      </Button>
    </form>
  );
}

export { SearchBar };
