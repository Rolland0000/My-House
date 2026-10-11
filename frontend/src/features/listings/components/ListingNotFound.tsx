import { useNavigate } from "react-router";
import { EmptyState } from "../../../shared/components";

function ListingNotFound() {
  const navigate = useNavigate();

  return (
    <div className="mx-auto max-w-2xl px-6 py-16">
      <EmptyState
        title="Listing not found"
        description="This listing doesn't exist, was removed by its owner, or isn't public."
        secondaryAction={{ label: "Back to listings", onClick: () => navigate("/") }}
      />
    </div>
  );
}

export { ListingNotFound };
