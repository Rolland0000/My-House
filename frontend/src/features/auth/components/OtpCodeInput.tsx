import { useEffect, useRef } from "react";
import type { ChangeEvent, ClipboardEvent, KeyboardEvent } from "react";

interface OtpCodeInputProps {
  length: number;
  value: string[];
  onChange: (value: string[]) => void;
  onComplete: (code: string) => void;
  disabled?: boolean;
}

function OtpCodeInput({
  length,
  value,
  onChange,
  onComplete,
  disabled = false,
}: OtpCodeInputProps) {
  const inputRefs = useRef<(HTMLInputElement | null)[]>([]);

  useEffect(() => {
    if (value.every((digit) => digit === "")) {
      inputRefs.current[0]?.focus();
    }
  }, [value]);

  function focusInput(index: number) {
    inputRefs.current[index]?.focus();
  }

  function setDigit(index: number, digit: string) {
    const next = [...value];
    next[index] = digit;
    onChange(next);

    if (digit && index < length - 1) {
      focusInput(index + 1);
    }
    if (next.every((d) => d !== "")) {
      onComplete(next.join(""));
    }
  }

  function handleChange(index: number, event: ChangeEvent<HTMLInputElement>) {
    const digit = event.target.value.replace(/\D/g, "").slice(-1);
    setDigit(index, digit);
  }

  function handleKeyDown(index: number, event: KeyboardEvent<HTMLInputElement>) {
    if (event.key === "Backspace" && !value[index] && index > 0) {
      focusInput(index - 1);
    }
  }

  function handlePaste(event: ClipboardEvent<HTMLInputElement>) {
    const pasted = event.clipboardData.getData("text").replace(/\D/g, "").slice(0, length);
    if (!pasted) return;
    event.preventDefault();

    const next = Array.from({ length }, (_, i) => pasted[i] ?? "");
    onChange(next);
    if (pasted.length === length) {
      onComplete(pasted);
    } else {
      focusInput(pasted.length);
    }
  }

  return (
    <div className="flex gap-2" role="group" aria-label="Code de vérification à 6 chiffres">
      {Array.from({ length }).map((_, index) => (
        <input
          key={index}
          ref={(el) => {
            inputRefs.current[index] = el;
          }}
          type="text"
          inputMode="numeric"
          autoComplete={index === 0 ? "one-time-code" : "off"}
          maxLength={1}
          value={value[index] ?? ""}
          disabled={disabled}
          onChange={(event) => handleChange(index, event)}
          onKeyDown={(event) => handleKeyDown(index, event)}
          onPaste={handlePaste}
          aria-label={`Chiffre ${index + 1}`}
          className="h-12 w-10 rounded-sm border border-border bg-surface text-center text-lg font-semibold text-text focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus disabled:cursor-not-allowed disabled:opacity-50"
        />
      ))}
    </div>
  );
}

export { OtpCodeInput };
export type { OtpCodeInputProps };
