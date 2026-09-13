import { useState } from "react";
import { Alert, Button, Spinner } from "../../../shared/components";
import { useOwnerRequestDocument } from "../hooks/useOwnerRequestDocument";
import type { AdminOwnerRequestDocument } from "../api";

interface DocumentViewerProps {
  requestId: string;
  doc: AdminOwnerRequestDocument;
}

/** Loads one document only once the admin asks for it — a detail page never
 *  downloads every identity document just by being opened. Images preview
 *  inline; a PDF surfaces as a link the admin clicks to open in a new tab —
 *  not an automatic `window.open`, which browsers block once it's no longer
 *  running inside the click that triggered the (async) download. */
function DocumentViewer({ requestId, doc }: DocumentViewerProps) {
  const [requested, setRequested] = useState(false);
  const { url, isPending, error } = useOwnerRequestDocument(requestId, doc.doc_id, requested);
  const isImage = doc.content_type.startsWith("image/");
  const label = doc.side ? `${doc.side} side` : "document";

  return (
    <div className="flex flex-col gap-2">
      <span className="text-sm font-semibold capitalize text-text">{label}</span>

      {!requested && (
        <Button variant="secondary" size="sm" onClick={() => setRequested(true)}>
          View {label}
        </Button>
      )}
      {requested && isPending && <Spinner size="sm" />}
      {requested && error && <Alert variant="error">Could not load this document.</Alert>}
      {requested && url && isImage && (
        <img
          src={url}
          alt={`Submitted identity document — ${label}`}
          className="max-w-xs rounded-sm border border-border"
        />
      )}
      {requested && url && !isImage && (
        <a
          href={url}
          target="_blank"
          rel="noreferrer"
          className="text-sm font-semibold text-primary underline"
        >
          Open document
        </a>
      )}
    </div>
  );
}

export { DocumentViewer };
