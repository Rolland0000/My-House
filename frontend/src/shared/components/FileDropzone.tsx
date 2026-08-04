import { useId, useRef, useState, type DragEvent } from "react";
import { UploadCloud } from "lucide-react";
import { cn } from "../utils/cn";

interface FileDropzoneProps {
  onFilesSelected: (files: File[]) => void;
  accept?: string;
  multiple?: boolean;
  hasError?: boolean;
  disabled?: boolean;
  label?: string;
  hint?: string;
  className?: string;
}

function FileDropzone({
  onFilesSelected,
  accept,
  multiple = false,
  hasError = false,
  disabled = false,
  label = "Glissez-déposez un fichier ici, ou cliquez pour sélectionner",
  hint,
  className,
}: FileDropzoneProps) {
  const [isDragActive, setIsDragActive] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);
  const inputId = useId();

  function handleFiles(fileList: FileList | null) {
    if (!fileList || fileList.length === 0) return;
    onFilesSelected(Array.from(fileList));
  }

  function handleDrop(event: DragEvent<HTMLDivElement>) {
    event.preventDefault();
    setIsDragActive(false);
    if (disabled) return;
    handleFiles(event.dataTransfer.files);
  }

  function handleDragOver(event: DragEvent<HTMLDivElement>) {
    event.preventDefault();
    if (disabled) return;
    setIsDragActive(true);
  }

  function handleDragLeave(event: DragEvent<HTMLDivElement>) {
    event.preventDefault();
    setIsDragActive(false);
  }

  return (
    <div
      role="button"
      tabIndex={disabled ? -1 : 0}
      aria-disabled={disabled}
      onClick={() => !disabled && inputRef.current?.click()}
      onKeyDown={(event) => {
        if (disabled) return;
        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          inputRef.current?.click();
        }
      }}
      onDrop={handleDrop}
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
      className={cn(
        "flex flex-col items-center justify-center gap-2 rounded-md border-2 border-dashed px-6 py-8 text-center",
        "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus",
        disabled
          ? "cursor-not-allowed border-border bg-surface opacity-50"
          : "cursor-pointer border-border bg-surface hover:bg-primary-soft",
        isDragActive && !disabled && "border-primary bg-primary-soft",
        hasError && "border-error",
        className
      )}
    >
      <UploadCloud className="size-6 text-text-muted" aria-hidden="true" />
      <p className="text-sm text-text">{label}</p>
      {hint && <p className="text-sm text-text-muted">{hint}</p>}

      <input
        ref={inputRef}
        id={inputId}
        type="file"
        accept={accept}
        multiple={multiple}
        disabled={disabled}
        className="sr-only"
        onChange={(event) => {
          handleFiles(event.target.files);
          event.target.value = "";
        }}
      />
    </div>
  );
}

export { FileDropzone };
export type { FileDropzoneProps };
