import { useState } from "react";
import type { FormEvent } from "react";
import { Link } from "react-router";
import {
  Alert,
  Button,
  Card,
  DimensionRule,
  FormField,
  Input,
  Spinner,
  useToast,
} from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { MAX_NAME_LENGTH, MAX_PHONE_LENGTH } from "../../../shared/api/constants";
import { formatInitials } from "../../../shared/utils/format";
import type { Profile } from "../api";
import { useProfile } from "../hooks/useProfile";
import {
  serverFieldError,
  validate,
  type ProfileFieldErrors as FieldErrors,
} from "../profileValidation";
import { AvatarUpload } from "./AvatarUpload";
import { DeleteAccountSection } from "./DeleteAccountSection";
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
        <h1 className="text-2xl font-bold text-ink-900">My profile</h1>
        <DimensionRule width={120} className="mt-3.5 mb-1" />
      </div>

      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
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
      </div>

      <FormField
        label="Email"
        hint="Email is how you sign in — changing it starts a new verification."
      >
        <Input value={profile.email} readOnly />
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

      <p className="text-sm text-text-muted">{ROLE_LABELS[profile.role]}</p>

      {bannerError && <Alert variant="error">{GENERIC_ERROR_MESSAGE}</Alert>}

      <Button type="submit" isLoading={update.isPending} className="self-start">
        Save changes
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
            <AvatarUpload
              avatarUrl={data.avatar_url ?? null}
              initials={formatInitials(data.first_name, data.last_name)}
            />
            <ProfileFields profile={data} />
            {data.role === "seeker" && (
              <div className="border-b border-border pb-6">
                <Link to="/owner-request" className="text-sm font-semibold text-primary underline">
                  Become an owner
                </Link>
              </div>
            )}
            <DeleteAccountSection isOwner={data.role === "owner"} />
          </div>
        )}
      </Card>
    </div>
  );
}

export { ProfileForm };
