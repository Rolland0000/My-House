import { MAX_NAME_LENGTH, MAX_PHONE_LENGTH } from "../../shared/api/constants";

export interface ProfileFieldErrors {
  firstName?: string;
  lastName?: string;
  phone?: string;
}

const FIELDS_BY_SERVER_NAME: [string, keyof ProfileFieldErrors][] = [
  ["first_name", "firstName"],
  ["last_name", "lastName"],
  ["phone", "phone"],
];

export const REQUIRED_MESSAGE = "This field is required.";

function maxLengthMessage(field: keyof ProfileFieldErrors): string {
  return `${field === "phone" ? MAX_PHONE_LENGTH : MAX_NAME_LENGTH} characters maximum.`;
}

/** Maps a server 400 back onto the field it names, restated in the form's own
 *  wording — the raw server message shouldn't reach the UI. An unrecognised
 *  rule returns null so it surfaces in the banner rather than as a
 *  mislabelled field error. */
export function serverFieldError(
  message: string
): { field: keyof ProfileFieldErrors; text: string } | null {
  const match = FIELDS_BY_SERVER_NAME.find(([name]) => message.includes(name));
  if (!match) return null;
  const [, field] = match;

  if (message.includes("at most")) return { field, text: maxLengthMessage(field) };
  if (message.includes("required") || message.includes("missing field")) {
    return { field, text: REQUIRED_MESSAGE };
  }
  return null;
}

export function validate(firstName: string, lastName: string, phone: string): ProfileFieldErrors {
  const errors: ProfileFieldErrors = {};
  if (firstName.trim().length > MAX_NAME_LENGTH) errors.firstName = maxLengthMessage("firstName");
  if (!lastName.trim()) errors.lastName = REQUIRED_MESSAGE;
  else if (lastName.trim().length > MAX_NAME_LENGTH) errors.lastName = maxLengthMessage("lastName");
  if (!phone.trim()) errors.phone = REQUIRED_MESSAGE;
  else if (phone.trim().length > MAX_PHONE_LENGTH) errors.phone = maxLengthMessage("phone");
  return errors;
}
