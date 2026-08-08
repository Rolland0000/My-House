import { Link, useParams } from "react-router";
import { ArrowLeft, MapPin } from "lucide-react";
import { Alert, Spinner } from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { formatPrice } from "../../../shared/utils/format";
import { useListing } from "../hooks/useListings";
import { typeLabels } from "../labels";

function ListingDetail() {
  const { id } = useParams<{ id: string }>();
  const { data, isPending, error } = useListing(id);

  if (isPending) {
    return (
      <div className="flex justify-center py-24">
        <Spinner size="lg" label="Chargement de l'annonce…" />
      </div>
    );
  }

  if (error instanceof ApiError && error.status === 404) {
    return (
      <div className="mx-auto flex max-w-2xl flex-col items-center gap-3 py-24 text-center">
        <h1 className="text-lg font-bold text-text">Ce bien n'est plus disponible</h1>
        <p className="text-text-muted">
          Il a peut-être été supprimé par le propriétaire ou son statut est passé à « non disponible
          ».
        </p>
        <Link to="/" className="font-semibold text-primary hover:underline">
          ← Retour au feed
        </Link>
      </div>
    );
  }

  if (error || !data) {
    return (
      <div className="mx-auto max-w-2xl p-6">
        <Alert variant="error" title="Impossible de charger cette annonce">
          {error instanceof ApiError ? error.message : "Réessayez dans quelques instants."}
        </Alert>
      </div>
    );
  }

  const listing = data.data;
  const location = [listing.city, listing.neighborhood].filter(Boolean).join(" · ");
  const ownerName = [listing.owner.first_name, listing.owner.last_name].filter(Boolean).join(" ");
  const stats = [
    listing.surface_m2 ? `${listing.surface_m2} m²` : null,
    listing.rooms ? `${listing.rooms} pièce${listing.rooms > 1 ? "s" : ""}` : null,
  ]
    .filter(Boolean)
    .join(" · ");

  return (
    <div className="mx-auto flex max-w-4xl flex-col gap-6 p-6">
      <Link
        to="/"
        className="flex w-fit items-center gap-1 text-sm font-semibold text-text-muted hover:text-text"
      >
        <ArrowLeft className="size-4" aria-hidden="true" />
        Retour au feed
      </Link>

      {listing.media.length > 0 ? (
        <div className="flex gap-3 overflow-x-auto rounded-md">
          {listing.media.map((media) => (
            <img
              key={media.id}
              src={media.url}
              alt=""
              className="h-64 w-96 shrink-0 rounded-md object-cover"
            />
          ))}
        </div>
      ) : (
        <div className="flex h-64 items-center justify-center rounded-md bg-primary-soft text-text-muted">
          Aucune photo
        </div>
      )}

      <div className="flex flex-col gap-4">
        <div className="flex flex-wrap items-start justify-between gap-2">
          <div>
            <h1 className="text-lg font-bold text-text">{listing.title}</h1>
            <p className="mt-1 flex items-center gap-1 text-text-muted">
              <MapPin className="size-4 shrink-0" aria-hidden="true" />
              {location}
            </p>
          </div>
          <span className="rounded-sm bg-primary-soft px-2 py-1 text-sm font-semibold text-text">
            {typeLabels[listing.type]}
          </span>
        </div>

        <div className="flex items-baseline gap-3">
          <span className="text-lg font-bold text-text">{formatPrice(listing.price)}</span>
          {stats && <span className="text-sm text-text-muted">{stats}</span>}
        </div>

        {listing.status === "unavailable" && (
          <Alert variant="warning">Ce bien n'est actuellement pas disponible.</Alert>
        )}

        <p className="whitespace-pre-line text-text">{listing.description}</p>

        <div className="flex items-center gap-3 border-t border-border pt-4">
          <div>
            <p className="font-semibold text-text">{ownerName || "Propriétaire"}</p>
            <p className="text-sm text-text-muted">Propriétaire</p>
          </div>
        </div>
      </div>
    </div>
  );
}

export { ListingDetail };
