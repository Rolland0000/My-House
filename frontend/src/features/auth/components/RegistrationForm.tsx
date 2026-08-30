import { useState } from "react";
import type { FormEvent } from "react";
import { Alert, Button, FormField, Input } from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { useRegister } from "../hooks/useRegister";
import { MAX_NAME_LENGTH, MAX_PHONE_LENGTH } from "../constants";

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
    if (!lastName.trim()) errors.lastName = "Ce champ est requis.";
    if (!phone.trim()) errors.phone = "Ce champ est requis.";
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
        <h2 className="text-lg font-bold text-text">Créez votre compte</h2>
        <p className="text-sm text-text-muted">Dernière étape avant de commencer.</p>
      </div>

      <FormField label="Prénom">
        <Input
          value={firstName}
          maxLength={MAX_NAME_LENGTH}
          onChange={(event) => setFirstName(event.target.value)}
          disabled={register.isPending}
        />
      </FormField>

      <FormField label="Nom" required error={fieldErrors.lastName}>
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

      <FormField label="Téléphone" required error={fieldErrors.phone}>
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
          Ce compte existe déjà.{" "}
          <button type="button" onClick={onAccountExists} className="font-semibold underline">
            Reconnectez-vous
          </button>
          .
        </Alert>
      )}

      {error && !alreadyExists && <Alert variant="error">{error.message}</Alert>}

      <Button type="submit" isLoading={register.isPending}>
        Terminer
      </Button>
    </form>
  );
}

export { RegistrationForm };
export type { RegistrationFormProps };
