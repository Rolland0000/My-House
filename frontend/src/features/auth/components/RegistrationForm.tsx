import { useState } from "react";
import type { FormEvent } from "react";
import { Alert, Button, FormField, Input } from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { MAX_NAME_LENGTH, MAX_PHONE_LENGTH } from "../../../shared/api/constants";
import { useRegister } from "../hooks/useRegister";

const REQUIRED_MESSAGE = "This field is required.";
const GENERIC_ERROR_MESSAGE = "An error occurred. Please try again.";

interface RegistrationFormProps {
  email: string;
  registrationTicket: string;
  onRegistered: () => void;
  onAccountExists: () => void;
}

interface FieldErrors {
  lastName?: string;
  phone?: string;
}

function RegistrationForm({
  email,
  registrationTicket,
  onRegistered,
  onAccountExists,
}: RegistrationFormProps) {
  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  const [phone, setPhone] = useState("");
  const [fieldErrors, setFieldErrors] = useState<FieldErrors>({});
  const register = useRegister();

  const error = register.error instanceof ApiError ? register.error : null;
  const alreadyExists = error?.code === "EMAIL_ALREADY_EXISTS";

  function handleSubmit(event: FormEvent) {
    event.preventDefault();
    const errors: FieldErrors = {};
    if (!lastName.trim()) errors.lastName = REQUIRED_MESSAGE;
    if (!phone.trim()) errors.phone = REQUIRED_MESSAGE;
    setFieldErrors(errors);
    if (Object.keys(errors).length > 0) return;

    register.mutate(
      {
        registration_ticket: registrationTicket,
        first_name: firstName.trim() || null,
        last_name: lastName.trim(),
        phone: phone.trim(),
      },
      { onSuccess: onRegistered }
    );
  }

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-5">
      <div>
        <h2 className="text-lg font-bold text-text">Create your account</h2>
        <p className="text-sm text-text-muted">Last step before you get started.</p>
      </div>

      <FormField label="First name">
        <Input
          value={firstName}
          maxLength={MAX_NAME_LENGTH}
          onChange={(event) => setFirstName(event.target.value)}
          disabled={register.isPending}
        />
      </FormField>

      <FormField label="Last name" required error={fieldErrors.lastName}>
        <Input
          value={lastName}
          maxLength={MAX_NAME_LENGTH}
          onChange={(event) => setLastName(event.target.value)}
          disabled={register.isPending}
          hasError={Boolean(fieldErrors.lastName)}
        />
      </FormField>

      <FormField label="Email">
        <Input value={email} disabled readOnly />
      </FormField>

      <FormField label="Phone" required error={fieldErrors.phone}>
        <Input
          type="tel"
          placeholder="+225 07 00 00 00 00"
          value={phone}
          maxLength={MAX_PHONE_LENGTH}
          onChange={(event) => setPhone(event.target.value)}
          disabled={register.isPending}
          hasError={Boolean(fieldErrors.phone)}
        />
      </FormField>

      {alreadyExists && (
        <Alert variant="warning">
          This account already exists.{" "}
          <button type="button" onClick={onAccountExists} className="font-semibold underline">
            Sign in
          </button>
          .
        </Alert>
      )}

      {error && !alreadyExists && <Alert variant="error">{GENERIC_ERROR_MESSAGE}</Alert>}

      <Button type="submit" isLoading={register.isPending}>
        Finish
      </Button>
    </form>
  );
}

export { RegistrationForm };
export type { RegistrationFormProps };
