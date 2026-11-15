export const MESSAGE_MIN_CHARS = 10;
export const MESSAGE_MAX_CHARS = 2000;
export const NAME_MAX_CHARS = 100;
export const EMAIL_MAX_CHARS = 255;

const EMAIL_PATTERN = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

export interface ContactMessageFields {
  name: string;
  email: string;
  message: string;
}

export type ContactMessageErrors = Partial<Record<keyof ContactMessageFields, string>>;

/** The same limits the API enforces, so a message is rejected before it is sent. */
export function validateContactMessage(fields: ContactMessageFields): ContactMessageErrors {
  const errors: ContactMessageErrors = {};
  const name = fields.name.trim();
  const email = fields.email.trim();
  const messageLength = [...fields.message.trim()].length;

  if (!name) errors.name = "Enter your name.";
  else if ([...name].length > NAME_MAX_CHARS) {
    errors.name = `Use at most ${NAME_MAX_CHARS} characters.`;
  }

  if (!EMAIL_PATTERN.test(email) || [...email].length > EMAIL_MAX_CHARS) {
    errors.email = "Enter a valid email address.";
  }

  if (messageLength < MESSAGE_MIN_CHARS) {
    errors.message = `Write at least ${MESSAGE_MIN_CHARS} characters.`;
  } else if (messageLength > MESSAGE_MAX_CHARS) {
    errors.message = `Use at most ${MESSAGE_MAX_CHARS} characters.`;
  }

  return errors;
}
