import { useEffect, useRef } from "react";
import { Link } from "react-router";
import { Alert, Button, buttonClasses } from "../../../shared/components";
import { cn } from "../../../shared/utils/cn";
import { contactErrorKind } from "../contactError";
import { useListingContact } from "../hooks/useListingContact";
import { PhoneNumber } from "./PhoneNumber";

const NOT_FOUND_MESSAGE = "This property is no longer available for contact.";
const RATE_LIMITED_MESSAGE = "You have revealed many numbers recently. Please try again later.";
const OTHER_ERROR_MESSAGE = "We couldn't load the numbers. Please try again.";

/** `hidden` covers the owner, an admin, and a viewer not known yet. */
export type ContactAccess = "sign-in" | "reveal" | "hidden";

interface ContactRevealProps {
  listingId: string;
  access: ContactAccess;
}

function ContactReveal({ listingId, access }: ContactRevealProps) {
  return (
    <div className="mt-4 flex flex-col gap-4">
      {access === "sign-in" && (
        <Link to="/login" className={cn(buttonClasses(), "w-fit")}>
          Sign in to contact
        </Link>
      )}
      {access === "reveal" && <OwnerContact listingId={listingId} />}
      {/* role="note": a role="alert" would be announced every time the page loads. */}
      <Alert variant="warning" role="note" title="Before you contact this owner">
        Never send money before visiting the property. My House does not hold deposits and is not a
        party to your agreement.
      </Alert>
    </div>
  );
}

function OwnerContact({ listingId }: { listingId: string }) {
  const { data: contact, error, isFetching, refetch } = useListingContact(listingId);
  const revealedRef = useRef<HTMLDivElement>(null);
  const focusOnReveal = useRef(false);

  // The button disappears on reveal, so focus moves to the numbers.
  useEffect(() => {
    if (contact && focusOnReveal.current) {
      focusOnReveal.current = false;
      revealedRef.current?.focus();
    }
  }, [contact]);

  function reveal() {
    focusOnReveal.current = true;
    void refetch();
  }

  if (contact) {
    return (
      <div
        ref={revealedRef}
        role="status"
        tabIndex={-1}
        className="flex flex-col gap-4 rounded-sm focus:outline-none focus-visible:ring-2 focus-visible:ring-focus"
      >
        <PhoneNumber label="Main number" phone={contact.phone} />
        {contact.secondary_phone && (
          <PhoneNumber label="Second number" phone={contact.secondary_phone} />
        )}
      </div>
    );
  }

  const errorKind = error ? contactErrorKind(error) : null;
  if (errorKind === "not-found") {
    return <Alert variant="info">{NOT_FOUND_MESSAGE}</Alert>;
  }
  if (errorKind === "other") {
    return (
      <Alert variant="error" title={OTHER_ERROR_MESSAGE}>
        <Button
          type="button"
          variant="secondary"
          size="sm"
          className="mt-2"
          isLoading={isFetching}
          onClick={reveal}
        >
          Try again
        </Button>
      </Alert>
    );
  }

  // A cached 401 is only seen after signing in again, so it shows the button.
  const isRateLimited = errorKind === "rate-limited";
  return (
    <>
      <Button
        type="button"
        className="w-fit"
        isLoading={isFetching}
        disabled={isRateLimited}
        onClick={reveal}
      >
        Contact the owner
      </Button>
      {isRateLimited && <Alert variant="warning">{RATE_LIMITED_MESSAGE}</Alert>}
    </>
  );
}

export { ContactReveal };
