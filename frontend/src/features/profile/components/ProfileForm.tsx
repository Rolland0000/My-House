import { useState } from "react";
import type { FormEvent } from "react";
import {
  Alert,
  Button,
  Card,
  FormField,
  Input,
  Spinner,
  useToast,
} from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { MAX_NAME_LENGTH, MAX_PHONE_LENGTH } from "../../../shared/api/constants";
import type { Profile } from "../api";
import { useProfile } from "../hooks/useProfile";
import { AvatarUpload } from "./AvatarUpload";
import { useUpdateProfile } from "../hooks/useUpdateProfile";

const ROLE_LABELS: Record<Profile["role"], string> = {
  seeker: "Locataire",
  owner: "Propriétaire",
  admin: "Administrateur",
};

interface FieldErrors {
  firstName?: string;
  lastName?: string;
  phone?: string;
}

const FIELDS_BY_SERVER_NAME: [string, keyof FieldErrors][] = [
  ["first_name", "firstName"],
  ["last_name", "lastName"],
  ["phone", "phone"],
];

const REQUIRED_MESSAGE = "Ce champ est requis.";

function maxLengthMessage(field: keyof FieldErrors): string {
  return `${field === "phone" ? MAX_PHONE_LENGTH : MAX_NAME_LENGTH} caractères maximum.`;
}

/** Maps a server 400 back onto the field it names, restated in the form's own
 *  wording — the raw server message is English. An unrecognised rule returns
 *  null so it surfaces in the banner rather than as a mislabelled field error. */
function serverFieldError(message: string): { field: keyof FieldErrors; text: string } | null {
  const match = FIELDS_BY_SERVER_NAME.find(([name]) => message.includes(name));
  if (!match) return null;
  const [, field] = match;

  if (message.includes("at most")) return { field, text: maxLengthMessage(field) };
  if (message.includes("required") || message.includes("missing field")) {
    return { field, text: REQUIRED_MESSAGE };
  }
  return null;
}

function validate(firstName: string, lastName: string, phone: string): FieldErrors {
  const errors: FieldErrors = {};
  if (firstName.trim().length > MAX_NAME_LENGTH) errors.firstName = maxLengthMessage("firstName");
  if (!lastName.trim()) errors.lastName = REQUIRED_MESSAGE;
  else if (lastName.trim().length > MAX_NAME_LENGTH) errors.lastName = maxLengthMessage("lastName");
  if (!phone.trim()) errors.phone = REQUIRED_MESSAGE;
  else if (phone.trim().length > MAX_PHONE_LENGTH) errors.phone = maxLengthMessage("phone");
  return errors;
}

interface ProfileFieldsProps {
  profile: Profile;
}

function ProfileFields({ profile }: ProfileFieldsProps) {
  const [firstName, setFirstName] = useState(profile.first_name ?? "");
  const [lastName, setLastName] = useState(profile.last_name ?? "");
  const [phone, setPhone] = useState(profile.phone ?? "");
  const [fieldErrors, setFieldErrors] = useState<FieldErrors>({});
  const { showToast } = useToast();
  const update = useUpdateProfile();

  const requestError = update.error instanceof ApiError ? update.error : null;
  const bannerError = requestError && !serverFieldError(requestError.message);

  function handleSubmit(event: FormEvent) {
    event.preventDefault();
    const errors = validate(firstName, lastName, phone);
    setFieldErrors(errors);
    if (Object.keys(errors).length > 0) return;

    update.mutate(
      {
        first_name: firstName.trim() || null,
        last_name: lastName.trim(),
        phone: phone.trim(),
      },
      {
        onSuccess: () => showToast("Profil mis à jour.", { variant: "success" }),
        onError: (error) => {
          if (!(error instanceof ApiError)) return;
          const fieldError = serverFieldError(error.message);
          if (fieldError) setFieldErrors({ [fieldError.field]: fieldError.text });
        },
      }
    );
  }

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-5">
      <div>
        <h1 className="text-lg font-bold text-text">Mon profil</h1>
        <p className="text-sm text-text-muted">Vos informations de contact.</p>
      </div>

      <FormField label="Prénom" error={fieldErrors.firstName}>
        <Input
          value={firstName}
          maxLength={MAX_NAME_LENGTH}
          onChange={(event) => setFirstName(event.target.value)}
          disabled={update.isPending}
          hasError={Boolean(fieldErrors.firstName)}
        />
      </FormField>

      <FormField label="Nom" required error={fieldErrors.lastName}>
        <Input
          value={lastName}
          maxLength={MAX_NAME_LENGTH}
          onChange={(event) => setLastName(event.target.value)}
          disabled={update.isPending}
          hasError={Boolean(fieldErrors.lastName)}
        />
      </FormField>

      <FormField label="Téléphone" required error={fieldErrors.phone}>
        <Input
          type="tel"
          placeholder="+225 07 00 00 00 00"
          value={phone}
          maxLength={MAX_PHONE_LENGTH}
          onChange={(event) => setPhone(event.target.value)}
          disabled={update.isPending}
          hasError={Boolean(fieldErrors.phone)}
        />
      </FormField>

      <dl className="flex flex-col gap-3 border-t border-border pt-4">
        <div className="flex flex-col gap-1.5">
          <dt className="text-sm font-semibold text-text">Email</dt>
          <dd className="text-base text-text-muted">{profile.email}</dd>
        </div>
        <div className="flex flex-col gap-1.5">
          <dt className="text-sm font-semibold text-text">Rôle</dt>
          <dd className="text-base text-text-muted">{ROLE_LABELS[profile.role]}</dd>
        </div>
      </dl>

      {bannerError && <Alert variant="error">{requestError.message}</Alert>}

      <Button type="submit" isLoading={update.isPending}>
        Enregistrer
      </Button>
    </form>
  );
}

function ProfileForm() {
  const { data, isPending, error } = useProfile();

  return (
    <div className="mx-auto w-full max-w-md px-4 py-12">
      <Card>
        {isPending && (
          <div className="flex justify-center py-8">
            <Spinner size="lg" label="Chargement du profil…" />
          </div>
        )}
        {!isPending && error && (
          <Alert variant="error">
            {error instanceof ApiError ? error.message : "Profil indisponible."}
          </Alert>
        )}
        {data && (
          <div className="flex flex-col gap-6">
            <AvatarUpload avatarUrl={data.avatar_url ?? null} />
            <ProfileFields profile={data} />
          </div>
        )}
      </Card>
    </div>
  );
}

export { ProfileForm };
