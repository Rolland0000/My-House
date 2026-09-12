import { useEffect, useRef, useState } from "react";
import { useForm } from "react-hook-form";
import { useNavigate } from "react-router";
import {
  Alert,
  Button,
  Card,
  DimensionRule,
  FileDropzone,
  FormField,
  Input,
  Select,
  Spinner,
} from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { MAX_NAME_LENGTH, MAX_PHONE_LENGTH } from "../../../shared/api/constants";
import { useProfile } from "../../profile/hooks/useProfile";
import { useSubmitOwnerRequest } from "../hooks/useSubmitOwnerRequest";
import { preCheckImage, preCheckPdf, serverMessage } from "../ownerRequestValidation";

type Modality = "photo" | "pdf";

interface FormValues {
  full_name: string;
  phone: string;
  secondary_phone: string;
  id_type: string;
  id_number: string;
}

const ID_TYPE_OPTIONS = [
  { value: "national_id", label: "National ID card" },
  { value: "passport", label: "Passport" },
  { value: "driving_license", label: "Driving license" },
];

/** A file paired with the object URL created for it, so the URL is never
 *  computed anywhere but the moment the file is selected. */
interface FileWithPreview {
  file: File;
  previewUrl: string;
}

interface ImageSlotProps {
  label: string;
  selection: FileWithPreview | null;
  error: string | null;
  disabled: boolean;
  onSelect: (file: File) => void;
  onRemove: () => void;
}

function ImageSlot({ label, selection, error, disabled, onSelect, onRemove }: ImageSlotProps) {
  // Cleanup only — the URL itself is created where the file is selected, not
  // here, so this effect never needs to call setState.
  useEffect(() => {
    if (!selection) return;
    return () => URL.revokeObjectURL(selection.previewUrl);
  }, [selection]);

  return (
    <div className="flex flex-col gap-2">
      <p className="text-sm font-semibold text-text">{label}</p>
      {selection ? (
        <div className="flex items-center gap-3 rounded-sm border border-border bg-surface p-2">
          <img src={selection.previewUrl} alt={label} className="size-14 rounded-sm object-cover" />
          <span className="flex-1 truncate text-sm text-text">{selection.file.name}</span>
          <Button
            type="button"
            variant="secondary"
            size="sm"
            onClick={onRemove}
            disabled={disabled}
          >
            Remove
          </Button>
        </div>
      ) : (
        <FileDropzone
          accept="image/jpeg,image/png,image/webp"
          capture="environment"
          disabled={disabled}
          hasError={Boolean(error)}
          label={`Take a photo or choose a file (${label.toLowerCase()})`}
          onFilesSelected={(files) => {
            const selected = files[0];
            if (selected) onSelect(selected);
          }}
        />
      )}
      {error && <p className="text-sm text-error">{error}</p>}
    </div>
  );
}

interface PdfSlotProps {
  file: File | null;
  error: string | null;
  disabled: boolean;
  onSelect: (file: File) => void;
  onRemove: () => void;
}

function PdfSlot({ file, error, disabled, onSelect, onRemove }: PdfSlotProps) {
  return (
    <div className="flex flex-col gap-2">
      {file ? (
        <div className="flex items-center gap-3 rounded-sm border border-border bg-surface p-2">
          <span className="flex-1 truncate text-sm text-text">
            {file.name} ({(file.size / (1024 * 1024)).toFixed(1)} MB)
          </span>
          <Button
            type="button"
            variant="secondary"
            size="sm"
            onClick={onRemove}
            disabled={disabled}
          >
            Remove
          </Button>
        </div>
      ) : (
        <FileDropzone
          accept="application/pdf"
          disabled={disabled}
          hasError={Boolean(error)}
          label="Choose a PDF file"
          onFilesSelected={(files) => {
            const selected = files[0];
            if (selected) onSelect(selected);
          }}
        />
      )}
      {error && <p className="text-sm text-error">{error}</p>}
    </div>
  );
}

function OwnerRequestForm() {
  const navigate = useNavigate();
  const { data: profile, isPending: profileLoading } = useProfile();
  const submit = useSubmitOwnerRequest();

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors },
  } = useForm<FormValues>({
    defaultValues: {
      full_name: "",
      phone: "",
      secondary_phone: "",
      id_type: "",
      id_number: "",
    },
  });

  // A ref, not state: writing it during the effect never triggers a render,
  // so it can't itself trigger the lint rule against setState-in-effect.
  const profilePrefilledRef = useRef(false);
  useEffect(() => {
    if (!profile || profilePrefilledRef.current) return;
    reset({
      full_name: [profile.first_name, profile.last_name].filter(Boolean).join(" "),
      phone: profile.phone ?? "",
      secondary_phone: "",
      id_type: "",
      id_number: "",
    });
    profilePrefilledRef.current = true;
  }, [profile, reset]);

  const [modality, setModality] = useState<Modality>("photo");
  const [front, setFront] = useState<FileWithPreview | null>(null);
  const [back, setBack] = useState<FileWithPreview | null>(null);
  const [pdfFile, setPdfFile] = useState<File | null>(null);
  const [frontError, setFrontError] = useState<string | null>(null);
  const [backError, setBackError] = useState<string | null>(null);
  const [pdfError, setPdfError] = useState<string | null>(null);
  const [documentsError, setDocumentsError] = useState<string | null>(null);

  function switchModality(next: Modality) {
    setModality(next);
    setFront(null);
    setBack(null);
    setPdfFile(null);
    setFrontError(null);
    setBackError(null);
    setPdfError(null);
    setDocumentsError(null);
  }

  const requestError = submit.error instanceof ApiError ? submit.error : null;

  function onSubmit(values: FormValues) {
    setDocumentsError(null);

    const documents = modality === "photo" ? [front?.file ?? null, back?.file ?? null] : [pdfFile];
    if (documents.some((file) => file === null)) {
      setDocumentsError(
        modality === "photo"
          ? "Both the front and back photos are required."
          : "A PDF file is required."
      );
      return;
    }

    const formData = new FormData();
    formData.append("phone", values.phone.trim());
    if (values.secondary_phone.trim()) {
      formData.append("secondary_phone", values.secondary_phone.trim());
    }
    formData.append(
      "identity_data",
      JSON.stringify({
        full_name: values.full_name.trim(),
        id_type: values.id_type,
        id_number: values.id_number.trim(),
      })
    );
    for (const file of documents as File[]) {
      formData.append("documents", file);
    }

    submit.mutate(formData, {
      onSuccess: () => navigate("/owner-request/status"),
    });
  }

  if (profileLoading) {
    return (
      <div className="flex justify-center py-16">
        <Spinner size="lg" label="Loading…" />
      </div>
    );
  }

  const disabled = submit.isPending;

  return (
    <div className="mx-auto w-full max-w-md px-4 py-12">
      <Card>
        <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col gap-5">
          <div>
            <h1 className="text-2xl font-bold text-ink-900">Become an owner</h1>
            <DimensionRule width={120} className="mt-3.5 mb-1" />
            <p className="text-sm text-text-muted">
              One form, one submission — your request is reviewed once sent and can't be edited
              afterwards.
            </p>
          </div>

          <FormField label="Phone" required error={errors.phone?.message}>
            <Input
              type="tel"
              placeholder="+225 07 00 00 00 00"
              maxLength={MAX_PHONE_LENGTH}
              disabled={disabled}
              hasError={Boolean(errors.phone)}
              {...register("phone", { required: "Phone is required." })}
            />
          </FormField>

          <FormField
            label="Secondary phone"
            hint="Optional — only sent if filled in."
            error={errors.secondary_phone?.message}
          >
            <Input
              type="tel"
              maxLength={MAX_PHONE_LENGTH}
              disabled={disabled}
              hasError={Boolean(errors.secondary_phone)}
              {...register("secondary_phone")}
            />
          </FormField>

          <FormField label="Full name" required error={errors.full_name?.message}>
            <Input
              maxLength={MAX_NAME_LENGTH}
              disabled={disabled}
              hasError={Boolean(errors.full_name)}
              {...register("full_name", { required: "Full name is required." })}
            />
          </FormField>

          <FormField label="ID type" required error={errors.id_type?.message}>
            <Select
              options={ID_TYPE_OPTIONS}
              placeholder="Select a type"
              disabled={disabled}
              hasError={Boolean(errors.id_type)}
              {...register("id_type", { required: "ID type is required." })}
            />
          </FormField>

          <FormField label="ID number" required error={errors.id_number?.message}>
            <Input
              maxLength={MAX_NAME_LENGTH}
              disabled={disabled}
              hasError={Boolean(errors.id_number)}
              {...register("id_number", { required: "ID number is required." })}
            />
          </FormField>

          <div className="flex flex-col gap-3">
            <p className="text-sm font-semibold text-text">
              ID document <span className="text-error">*</span>
            </p>
            <div className="flex gap-2">
              <Button
                type="button"
                variant={modality === "photo" ? "primary" : "secondary"}
                size="sm"
                disabled={disabled}
                onClick={() => switchModality("photo")}
              >
                Two photos
              </Button>
              <Button
                type="button"
                variant={modality === "pdf" ? "primary" : "secondary"}
                size="sm"
                disabled={disabled}
                onClick={() => switchModality("pdf")}
              >
                One PDF
              </Button>
            </div>

            {modality === "photo" ? (
              <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
                <ImageSlot
                  label="Front"
                  selection={front}
                  error={frontError}
                  disabled={disabled}
                  onSelect={(file) => {
                    const problem = preCheckImage(file);
                    setFrontError(problem);
                    if (!problem) setFront({ file, previewUrl: URL.createObjectURL(file) });
                  }}
                  onRemove={() => {
                    setFront(null);
                    setFrontError(null);
                  }}
                />
                <ImageSlot
                  label="Back"
                  selection={back}
                  error={backError}
                  disabled={disabled}
                  onSelect={(file) => {
                    const problem = preCheckImage(file);
                    setBackError(problem);
                    if (!problem) setBack({ file, previewUrl: URL.createObjectURL(file) });
                  }}
                  onRemove={() => {
                    setBack(null);
                    setBackError(null);
                  }}
                />
              </div>
            ) : (
              <PdfSlot
                file={pdfFile}
                error={pdfError}
                disabled={disabled}
                onSelect={(file) => {
                  const problem = preCheckPdf(file);
                  setPdfError(problem);
                  if (!problem) setPdfFile(file);
                }}
                onRemove={() => {
                  setPdfFile(null);
                  setPdfError(null);
                }}
              />
            )}
            {documentsError && <p className="text-sm text-error">{documentsError}</p>}
          </div>

          {requestError && <Alert variant="error">{serverMessage(requestError)}</Alert>}

          <Button type="submit" isLoading={submit.isPending} className="self-start">
            Submit request
          </Button>
        </form>
      </Card>
    </div>
  );
}

export { OwnerRequestForm };
