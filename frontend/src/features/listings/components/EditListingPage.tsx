import { useNavigate, useParams } from "react-router";
import { Alert, Spinner } from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { useProfile } from "../../profile";
import { useListing } from "../hooks/useListings";
import { useUpdateListing } from "../hooks/useUpdateListing";
import { listingDetailToFormValues } from "../listingFormValidation";
import { ListingForm } from "./ListingForm";
import { ListingNotFound } from "./ListingNotFound";

function isNotFound(error: unknown) {
  return error instanceof ApiError && error.status === 404;
}

function EditListingPage() {
  const { id = "" } = useParams<"id">();
  const navigate = useNavigate();
  const { data, isPending, error } = useListing(id);
  const { data: profile, isPending: isProfilePending, error: profileError } = useProfile();
  const mutation = useUpdateListing(id);

  if (isPending || isProfilePending) {
    return (
      <div className="flex justify-center py-24">
        <Spinner size="lg" label="Loading listing…" />
      </div>
    );
  }

  if (isNotFound(error) || isNotFound(mutation.error)) {
    return <ListingNotFound />;
  }

  if (error || profileError || !data || !profile) {
    return (
      <div className="mx-auto max-w-2xl p-6">
        <Alert variant="error" title="Unable to load this listing">
          Please try again in a moment.
        </Alert>
      </div>
    );
  }

  // A published listing is readable by anyone, so ownership is checked here too;
  // the PUT still answers 404 for someone else's listing.
  if (profile.id !== data.data.owner.id) {
    return <ListingNotFound />;
  }

  return (
    <ListingForm
      // Remount per listing so the one-time prefill runs again if the id changes.
      key={id}
      mode="edit"
      initialValues={listingDetailToFormValues(data.data)}
      mutation={mutation}
      onCancel={() => navigate(`/listings/${id}`)}
    />
  );
}

export { EditListingPage };
