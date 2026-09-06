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
import {
  serverFieldError,
  validate,
  type ProfileFieldErrors as FieldErrors,
} from "../profileValidation";
import { AvatarUpload } from "./AvatarUpload";
import { useUpdateProfile } from "../hooks/useUpdateProfile";

const ROLE_LABELS: Record<Profile["role"], string> = {
  seeker: "Tenant",
  owner: "Owner",
  admin: "Administrator",
};

const GENERIC_ERROR_MESSAGE = "An error occurred. Please try again.";

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
        onSuccess: () => showToast("Profile updated.", { variant: "success" }),
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
        <h1 className="text-lg font-bold text-text">My profile</h1>
        <p className="text-sm text-text-muted">Your contact information.</p>
      </div>

      <FormField label="First name" error={fieldErrors.firstName}>
        <Input
          value={firstName}
          maxLength={MAX_NAME_LENGTH}
          onChange={(event) => setFirstName(event.target.value)}
          disabled={update.isPending}
          hasError={Boolean(fieldErrors.firstName)}
        />
      </FormField>

      <FormField label="Last name" required error={fieldErrors.lastName}>
        <Input
          value={lastName}
          maxLength={MAX_NAME_LENGTH}
          onChange={(event) => setLastName(event.target.value)}
          disabled={update.isPending}
          hasError={Boolean(fieldErrors.lastName)}
        />
      </FormField>

      <FormField label="Phone" required error={fieldErrors.phone}>
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
          <dt className="text-sm font-semibold text-text">Role</dt>
          <dd className="text-base text-text-muted">{ROLE_LABELS[profile.role]}</dd>
        </div>
      </dl>

      {bannerError && <Alert variant="error">{GENERIC_ERROR_MESSAGE}</Alert>}

      <Button type="submit" isLoading={update.isPending}>
        Save
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
            <Spinner size="lg" label="Loading profile…" />
          </div>
        )}
        {!isPending && error && (
          <Alert variant="error">
            {error instanceof ApiError ? GENERIC_ERROR_MESSAGE : "Profile unavailable."}
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
