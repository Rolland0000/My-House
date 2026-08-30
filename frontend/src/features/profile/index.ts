// ProfileForm is deliberately not re-exported: router.tsx lazy-loads it by
// path, and a static re-export here would pull it into the eager bundle.

export { useProfile } from "./hooks/useProfile";
export { useUpdateProfile } from "./hooks/useUpdateProfile";
export { getMe, updateMe, profileQueryKey, type Profile, type UpdateProfilePayload } from "./api";
