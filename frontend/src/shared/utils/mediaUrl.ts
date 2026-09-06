const HTTP_URL_PATTERN = /^https?:/i;
const HTTP_OR_BLOB_URL_PATTERN = /^(https?|blob):/i;

/** Remote media (listing photos, stored avatars) must be http(s) — never
 *  `javascript:`/`data:`, which this regex-based guard rules out directly. */
export function isRemoteMediaUrl(url: string): boolean {
  return HTTP_URL_PATTERN.test(url);
}

/** Same as `isRemoteMediaUrl`, plus `blob:` for a local object-URL preview
 *  (e.g. an avatar selected but not yet uploaded). */
export function isDisplayableMediaUrl(url: string): boolean {
  return HTTP_OR_BLOB_URL_PATTERN.test(url);
}
