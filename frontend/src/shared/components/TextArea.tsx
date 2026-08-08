import type { TextareaHTMLAttributes } from "react";
import { cn } from "../utils/cn";

interface TextAreaProps extends TextareaHTMLAttributes<HTMLTextAreaElement> {
  hasError?: boolean;
}

function TextArea({ hasError = false, className, rows = 4, ...rest }: TextAreaProps) {
  return (
    <textarea
      rows={rows}
      className={cn(
        "w-full resize-y rounded-sm border bg-surface px-3 py-2 text-base text-text placeholder:text-text-muted " +
          "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus " +
          "disabled:cursor-not-allowed disabled:opacity-50",
        hasError ? "border-error" : "border-border",
        className
      )}
      {...rest}
    />
  );
}

export { TextArea };
export type { TextAreaProps };
