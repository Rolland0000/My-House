import { useState } from "react";
import type { FormEvent } from "react";
import { Alert, Button, FormField, Input } from "../../../shared/components";
import { useUpdateProfile } from "../hooks/useUpdateProfile";

interface ProfileSetupFormProps {
  email: string;
  onDone: () => void;
}

interface FieldErrors {
  firstName?: string;
  lastName?: string;
}

function ProfileSetupForm({ email, onDone }: ProfileSetupFormProps) {
  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  const [phone, setPhone] = useState("");
  const [fieldErrors, setFieldErrors] = useState<FieldErrors>({});
  const updateProfile = useUpdateProfile();

  function handleSubmit(event: FormEvent) {
    event.preventDefault();
    const errors: FieldErrors = {};
    if (!firstName.trim()) errors.firstName = "Ce champ est requis.";
    if (!lastName.trim()) errors.lastName = "Ce champ est requis.";
    setFieldErrors(errors);
    if (Object.keys(errors).length > 0) return;

    updateProfile.mutate(
      { first_name: firstName.trim(), last_name: lastName.trim(), phone: phone.trim() || null },
      { onSuccess: onDone }
    );
  }

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-5">
      <div>
        <h2 className="text-lg font-bold text-text">Complétez votre profil</h2>
        <p className="text-sm text-text-muted">Dernière étape avant de commencer.</p>
      </div>

      <FormField label="Prénom" required error={fieldErrors.firstName}>
        <Input
          value={firstName}
          onChange={(event) => setFirstName(event.target.value)}
          disabled={updateProfile.isPending}
          hasError={Boolean(fieldErrors.firstName)}
        />
      </FormField>

      <FormField label="Nom" required error={fieldErrors.lastName}>
        <Input
          value={lastName}
          onChange={(event) => setLastName(event.target.value)}
          disabled={updateProfile.isPending}
          hasError={Boolean(fieldErrors.lastName)}
        />
      </FormField>

      <div className="flex flex-col gap-1.5">
        <span className="text-sm font-semibold text-text">Email</span>
        <div className="flex items-center gap-2">
          <Input value={email} disabled readOnly />
          <span className="shrink-0 rounded-sm bg-success-soft px-2 py-1 text-sm font-semibold text-success">
            vérifié
          </span>
        </div>
      </div>

      <FormField
        label="Téléphone (facultatif)"
        hint="Vous pourrez aussi le renseigner plus tard, lors d'une demande propriétaire."
      >
        <Input
          type="tel"
          placeholder="+225 07 00 00 00 00"
          value={phone}
          onChange={(event) => setPhone(event.target.value)}
          disabled={updateProfile.isPending}
        />
      </FormField>

      {updateProfile.isError && <Alert variant="error">Une erreur est survenue, réessayez.</Alert>}

      <Button type="submit" isLoading={updateProfile.isPending}>
        Terminer
      </Button>
    </form>
  );
}

export { ProfileSetupForm };
export type { ProfileSetupFormProps };
