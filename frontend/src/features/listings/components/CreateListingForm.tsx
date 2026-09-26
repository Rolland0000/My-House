import { useForm } from "react-hook-form";
import { useNavigate } from "react-router";
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
import { useCreateListing } from "../hooks/useCreateListing";
import { typeLabels } from "../labels";
import type { ListingType } from "../api";
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
  toCreateListingPayload,
  type ListingFormValues,
} from "../listingFormValidation";

const TYPE_OPTIONS = (Object.keys(typeLabels) as ListingType[]).map((value) => ({
  value,
  label: typeLabels[value],
}));

function CreateListingForm() {
  const navigate = useNavigate();
  const mutation = useCreateListing();

  const {
    register,
    handleSubmit,
    setError,
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

  const requestError = mutation.error instanceof ApiError ? mutation.error : null;

  function onSubmit(values: ListingFormValues) {
    mutation.mutate(toCreateListingPayload(values), {
      onSuccess: (response) => {
        // MH-58 will replace this target with the new listing's photo screen.
        navigate(`/listings/${response.data.id}`);
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
              <h1 className="text-2xl font-bold text-ink-900">Publish a listing</h1>
              <DimensionRule width={120} className="mt-3.5 mb-1" />
              <p className="text-sm text-text-muted">
                Fill in the details below. You'll add photos on the next screen.
              </p>
            </div>

            {requestError && <Alert variant="error">{requestErrorMessage(requestError)}</Alert>}

            <FormField label="Title" required error={errors.title?.message}>
              <Input
                hasError={Boolean(errors.title)}
                {...register("title", {
                  required: "Title is required.",
                  minLength: {
                    value: TITLE_MIN_LENGTH,
                    message: `${TITLE_MIN_LENGTH} characters minimum.`,
                  },
                  maxLength: {
                    value: TITLE_MAX_LENGTH,
                    message: `${TITLE_MAX_LENGTH} characters maximum.`,
                  },
                })}
              />
            </FormField>

            <FormField label="Description" required error={errors.description?.message}>
              <TextArea
                rows={5}
                hasError={Boolean(errors.description)}
                {...register("description", {
                  required: "Description is required.",
                  minLength: {
                    value: DESCRIPTION_MIN_LENGTH,
                    message: `${DESCRIPTION_MIN_LENGTH} characters minimum.`,
                  },
                  maxLength: {
                    value: DESCRIPTION_MAX_LENGTH,
                    message: `${DESCRIPTION_MAX_LENGTH} characters maximum.`,
                  },
                })}
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

            <FormField label="Price (XAF)" required error={errors.price?.message}>
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

            <Button type="submit" isLoading={mutation.isPending} className="self-start">
              Publish listing
            </Button>
          </fieldset>
        </form>
      </Card>
    </div>
  );
}

export { CreateListingForm };
