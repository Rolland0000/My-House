import { useState } from "react";

/** Editable draft of a value held in the URL, replaced whenever that URL value changes. */
export function useUrlDraft(urlValue: string) {
  const [draft, setDraft] = useState(urlValue);
  const [prevUrlValue, setPrevUrlValue] = useState(urlValue);

  if (urlValue !== prevUrlValue) {
    setPrevUrlValue(urlValue);
    setDraft(urlValue);
  }

  return [draft, setDraft] as const;
}
