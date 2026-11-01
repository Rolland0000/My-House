import { useEffect, useState } from "react";
import { Check, Copy, MessageCircle, Phone } from "lucide-react";
import { Button, buttonClasses } from "../../../shared/components";
import { telHref, whatsappHref } from "../phoneLinks";

const COPIED_FEEDBACK_MS = 2000;

const actionLinkClasses = buttonClasses("secondary", "sm");

type CopyState = "idle" | "copied" | "failed";

interface PhoneNumberProps {
  label: string;
  phone: string;
}

function PhoneNumber({ label, phone }: PhoneNumberProps) {
  const [copyState, setCopyState] = useState<CopyState>("idle");
  const whatsapp = whatsappHref(phone);
  const copied = copyState === "copied";

  useEffect(() => {
    if (!copied) return;
    const timer = window.setTimeout(() => setCopyState("idle"), COPIED_FEEDBACK_MS);
    return () => window.clearTimeout(timer);
  }, [copied]);

  async function copy() {
    // Throws outside a secure context, where `navigator.clipboard` is missing, and rejects
    // when access is denied.
    try {
      await navigator.clipboard.writeText(phone);
      setCopyState("copied");
    } catch {
      setCopyState("failed");
    }
  }

  return (
    <div className="flex flex-col gap-2">
      <div>
        <p className="text-sm text-text-muted">{label}</p>
        <p className="select-all text-lg font-semibold text-ink-900">{phone}</p>
      </div>
      <div className="flex flex-wrap gap-2">
        <a href={telHref(phone)} aria-label={`Call ${phone}`} className={actionLinkClasses}>
          <Phone className="size-4" aria-hidden="true" />
          Call
        </a>
        <Button
          type="button"
          variant="secondary"
          size="sm"
          onClick={copy}
          aria-label={copied ? `Copied ${phone}` : `Copy ${phone}`}
          leftIcon={
            copied ? (
              <Check className="size-4" aria-hidden="true" />
            ) : (
              <Copy className="size-4" aria-hidden="true" />
            )
          }
        >
          {copied ? "Copied" : "Copy"}
        </Button>
        {whatsapp && (
          <a
            href={whatsapp}
            target="_blank"
            rel="noopener noreferrer"
            aria-label={`Open WhatsApp chat with ${phone}`}
            className={actionLinkClasses}
          >
            <MessageCircle className="size-4" aria-hidden="true" />
            WhatsApp
          </a>
        )}
      </div>
      {copyState === "failed" && (
        <p className="text-sm text-error-text">
          Copying is not available in this browser. Select the number to copy it.
        </p>
      )}
    </div>
  );
}

export { PhoneNumber };
