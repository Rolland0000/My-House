import { useNavigate } from "react-router";
import { EmptyState } from "../../../shared/components";

function ListingNotFound() {
  const navigate = useNavigate();

  return (
    <div className="mx-auto max-w-2xl px-6 py-16">
      <EmptyState
        title="This property is no longer available"
        description="It may have been removed by the owner, or its status changed to “unavailable”."
        secondaryAction={{ label: "Back to listings", onClick: () => navigate("/") }}
      />
    </div>
  );
}

export { ListingNotFound };
