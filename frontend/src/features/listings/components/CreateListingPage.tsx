import { useCreateListing } from "../hooks/useCreateListing";
import { ListingForm } from "./ListingForm";

function CreateListingPage() {
  const mutation = useCreateListing();
  return <ListingForm mode="create" mutation={mutation} />;
}

export { CreateListingPage };
