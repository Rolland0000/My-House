import { useEffect, useRef } from "react";
import { useForm } from "react-hook-form";
import { useNavigate } from "react-router";
import type { UseMutationResult } from "@tanstack/react-query";
import {
  Alert,
  Button,
  Card,
  DimensionRule,
  FormField,
  Input,
  Select,
  TextArea,
} from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { typeLabels } from "../labels";
import type { ListingDetail, ListingRequest, ListingType } from "../api";
import {
  DESCRIPTION_MAX_LENGTH,
  DESCRIPTION_MIN_LENGTH,
  PRICE_MAX,
  PRICE_MIN,
  ROOMS_MAX,
  ROOMS_MIN,
  SURFACE_M2_MAX,
  SURFACE_M2_MIN,
  TITLE_MAX_LENGTH,
  TITLE_MIN_LENGTH,
  integerFieldRule,
  placeNameFieldRule,
  requestErrorMessage,
  serverFieldToFormField,
  toListingPayload,
  trimmedTextFieldRule,
  type ListingFormValues,
} from "../listingFormValidation";

const TYPE_OPTIONS = (Object.keys(typeLabels) as ListingType[]).map((value) => ({
  value,
  label: typeLabels[value],
}));

type ListingFormMode = "create" | "edit";

const MODE_TEXT: Record<ListingFormMode, { title: string; intro: string; submit: string }> = {
  create: {
    title: "Publish a listing",
    intro:
      "Fill in the details below. The listing is saved as a draft. You'll add photos and publish it from its page.",
    submit: "Publish listing",
  },
  edit: {
    title: "Edit listing",
    intro: "Update the details below. Saving doesn't change whether the listing is published.",
    submit: "Save changes",
  },
};

interface ListingFormProps {
  mode: ListingFormMode;
  mutation: UseMutationResult<{ data: ListingDetail }, Error, ListingRequest>;
  initialValues?: ListingFormValues;
  onCancel?: () => void;
}

function ListingForm({ mode, mutation, initialValues, onCancel }: ListingFormProps) {
  const navigate = useNavigate();
  const text = MODE_TEXT[mode];

  const {
    register,
    handleSubmit,
    setError,
    reset,
    formState: { errors },
  } = useForm<ListingFormValues>({
    defaultValues: {
      title: "",
      description: "",
      type: "",
      city: "",
      neighborhood: "",
    },
  });

  // Prefill once: a later refetch of the listing must not wipe what the user typed.
  const hasPrefilled = useRef(false);
  useEffect(() => {
    if (!initialValues || hasPrefilled.current) return;
    hasPrefilled.current = true;
    reset(initialValues);
  }, [initialValues, reset]);

  const requestError = mutation.error instanceof ApiError ? mutation.error : null;

  function onSubmit(values: ListingFormValues) {
    mutation.mutate(toListingPayload(values), {
      onSuccess: (response) => {
        // Replace in edit mode so Back from the detail page doesn't reopen the submitted form.
        navigate(`/listings/${response.data.id}`, { replace: mode === "edit" });
      },
      onError: (error) => {
        if (!(error instanceof ApiError) || error.code !== "VALIDATION_FAILED") return;
        for (const fieldError of error.fieldErrors) {
          const formField = serverFieldToFormField(fieldError.field);
          if (formField) setError(formField, { type: "server", message: fieldError.message });
        }
      },
    });
  }

  const disabled = mutation.isPending;

  return (
    <div className="mx-auto w-full max-w-2xl px-4 py-12">
      <Card>
        <form onSubmit={handleSubmit(onSubmit)}>
          <fieldset disabled={disabled} className="flex flex-col gap-5">
            <div>
              <h1 className="text-2xl font-bold text-ink-900">{text.title}</h1>
              <DimensionRule width={120} className="mt-3.5 mb-1" />
              <p className="text-sm text-text-muted">{text.intro}</p>
            </div>

            {requestError && (
              <Alert variant="error">
                {requestErrorMessage(requestError)}
                {requestError.fieldErrors.length > 0 && (
                  <ul className="mt-1 list-disc pl-5">
                    {requestError.fieldErrors.map((fieldError) => (
                      <li key={`${fieldError.field}-${fieldError.message}`}>
                        {fieldError.message}
                      </li>
                    ))}
                  </ul>
                )}
              </Alert>
            )}

            <FormField label="Title" required error={errors.title?.message}>
              <Input
                hasError={Boolean(errors.title)}
                {...register(
                  "title",
                  trimmedTextFieldRule("Title", TITLE_MIN_LENGTH, TITLE_MAX_LENGTH)
                )}
              />
            </FormField>

            <FormField label="Description" required error={errors.description?.message}>
              <TextArea
                rows={5}
                hasError={Boolean(errors.description)}
                {...register(
                  "description",
                  trimmedTextFieldRule(
                    "Description",
                    DESCRIPTION_MIN_LENGTH,
                    DESCRIPTION_MAX_LENGTH
                  )
                )}
              />
            </FormField>

            <FormField label="Type" required error={errors.type?.message}>
              <Select
                options={TYPE_OPTIONS}
                placeholder="Select a type"
                hasError={Boolean(errors.type)}
                {...register("type", { required: "Type is required." })}
              />
            </FormField>

            <FormField label="Monthly rent (XAF)" required error={errors.price?.message}>
              <Input
                type="number"
                step={1}
                inputMode="numeric"
                hasError={Boolean(errors.price)}
                {...register("price", {
                  required: "Price is required.",
                  ...integerFieldRule(PRICE_MIN, PRICE_MAX, " XAF"),
                })}
              />
            </FormField>

            <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
              <FormField label="Surface (m²)" hint="Optional" error={errors.surfaceM2?.message}>
                <Input
                  type="number"
                  step={1}
                  inputMode="numeric"
                  hasError={Boolean(errors.surfaceM2)}
                  {...register(
                    "surfaceM2",
                    integerFieldRule(SURFACE_M2_MIN, SURFACE_M2_MAX, " m²")
                  )}
                />
              </FormField>

              <FormField label="Rooms" hint="Optional" error={errors.rooms?.message}>
                <Input
                  type="number"
                  step={1}
                  inputMode="numeric"
                  hasError={Boolean(errors.rooms)}
                  {...register("rooms", integerFieldRule(ROOMS_MIN, ROOMS_MAX))}
                />
              </FormField>
            </div>

            <FormField label="City" required error={errors.city?.message}>
              <Input
                hasError={Boolean(errors.city)}
                {...register("city", placeNameFieldRule("City"))}
              />
            </FormField>

            <FormField label="Neighborhood" required error={errors.neighborhood?.message}>
              <Input
                hasError={Boolean(errors.neighborhood)}
                {...register("neighborhood", placeNameFieldRule("Neighborhood"))}
              />
            </FormField>

            <div className="flex gap-3">
              <Button type="submit" isLoading={mutation.isPending}>
                {text.submit}
              </Button>
              {onCancel && (
                <Button type="button" variant="secondary" onClick={onCancel}>
                  Cancel
                </Button>
              )}
            </div>
          </fieldset>
        </form>
      </Card>
    </div>
  );
}

export { ListingForm };
export type { ListingFormProps };
