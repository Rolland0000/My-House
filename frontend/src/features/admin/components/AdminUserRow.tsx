import { Link } from "react-router";
import { Badge } from "../../../shared/components";
import { formatFullName } from "../../../shared/utils/format";
import { ROLE_BADGE, userStateBadge } from "../adminUserBadges";
import type { AdminUser } from "../api";
import { AccountStateAction } from "./AccountStateAction";
import { DeleteUserAction } from "./DeleteUserAction";
import { RevokeOwnerAction } from "./RevokeOwnerAction";

const NO_VALUE = "—";
const CELL_CLASSES = "px-4 py-3 align-middle";

interface AdminUserRowProps {
  user: AdminUser;
}

function AdminUserRow({ user }: AdminUserRowProps) {
  const role = ROLE_BADGE[user.role];
  const state = userStateBadge(user.is_active);

  return (
    <tr className="border-b border-border last:border-b-0">
      <td className={`${CELL_CLASSES} font-semibold text-text`}>
        {formatFullName(user.first_name, user.last_name) || NO_VALUE}
      </td>
      <td className={`${CELL_CLASSES} text-text-muted`}>{user.email}</td>
      <td className={`${CELL_CLASSES} whitespace-nowrap text-text-muted`}>
        {user.phone ?? NO_VALUE}
      </td>
      <td className={CELL_CLASSES}>
        <Badge tone={role.tone}>{role.label}</Badge>
      </td>
      <td className={CELL_CLASSES}>
        <Badge tone={state.tone}>{state.label}</Badge>
      </td>
      <td className={`${CELL_CLASSES} text-text`}>
        {user.role === "owner" ? (
          <Link
            to={`/admin/listings?owner_id=${user.id}`}
            aria-label={`${user.listing_count} listing${user.listing_count === 1 ? "" : "s"} of ${user.email}`}
            className="font-semibold underline hover:text-ink-900"
          >
            {user.listing_count}
          </Link>
        ) : (
          user.listing_count
        )}
      </td>
      <td className={`${CELL_CLASSES} whitespace-nowrap text-text-muted`}>
        {new Date(user.created_at).toLocaleDateString()}
      </td>
      <td className={CELL_CLASSES}>
        {user.role !== "admin" && (
          <div className="flex items-center gap-4">
            {user.role === "owner" && <RevokeOwnerAction user={user} />}
            <AccountStateAction user={user} />
            <DeleteUserAction user={user} />
          </div>
        )}
      </td>
    </tr>
  );
}

export { AdminUserRow };
