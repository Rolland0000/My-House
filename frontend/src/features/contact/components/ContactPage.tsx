import { useState } from "react";
import type { FormEvent, ReactNode } from "react";
import { Clock, Mail, MapPin, Phone } from "lucide-react";
import { Alert, Button, Card, FormField, Input, TextArea } from "../../../shared/components";
import { ApiError } from "../../../shared/api/client";
import { COMPANY_INFO } from "../companyInfo";
import {
  EMAIL_MAX_CHARS,
  MESSAGE_MAX_CHARS,
  NAME_MAX_CHARS,
  validateContactMessage,
  type ContactMessageErrors,
} from "../contactMessageValidation";
import { useSendContactMessage } from "../hooks/useSendContactMessage";

const RATE_LIMITED_MESSAGE =
  "You have already written to us recently. Please try again in a minute.";
const GENERIC_ERROR_MESSAGE = "We couldn't send your message. Please try again.";

function errorMessage(error: unknown): string {
  return error instanceof ApiError && error.status === 429
    ? RATE_LIMITED_MESSAGE
    : GENERIC_ERROR_MESSAGE;
}

interface DetailProps {
  icon: typeof Mail;
  label: string;
  children: ReactNode;
}

function Detail({ icon: Icon, label, children }: DetailProps) {
  return (
    <li className="flex gap-3">
      <Icon className="mt-0.5 size-5 flex-none text-ink-500" aria-hidden="true" />
      <div>
        <p className="text-sm font-semibold text-ink-900">{label}</p>
        <div className="text-sm text-text">{children}</div>
      </div>
    </li>
  );
}

function ContactPage() {
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [message, setMessage] = useState("");
  const [errors, setErrors] = useState<ContactMessageErrors>({});
  const sendMessage = useSendContactMessage();

  function handleSubmit(event: FormEvent) {
    event.preventDefault();
    const found = validateContactMessage({ name, email, message });
    setErrors(found);
    if (Object.keys(found).length > 0) return;

    sendMessage.mutate(
      { name: name.trim(), email: email.trim(), message: message.trim() },
      {
        onSuccess: () => {
          setName("");
          setEmail("");
          setMessage("");
        },
      }
    );
  }

  return (
    <div className="mx-auto w-full max-w-[1112px] px-4 py-10 sm:px-7">
      <header className="mb-8 max-w-2xl">
        <h1 className="text-3xl font-bold text-ink-900">Contact us</h1>
        <p className="mt-2 text-text-muted">
          A question about a listing, your account or the platform? Write to us and we will answer
          by email.
        </p>
      </header>

      <div className="grid gap-8 md:grid-cols-[1fr_1.4fr]">
        <Card>
          <h2 className="mb-4 text-lg font-semibold text-ink-900">Our details</h2>
          <ul className="flex flex-col gap-4">
            <Detail icon={Mail} label="Email">
              <a href={`mailto:${COMPANY_INFO.email}`} className="underline hover:text-ink-900">
                {COMPANY_INFO.email}
              </a>
            </Detail>
            <Detail icon={Phone} label="Phone">
              <a href={COMPANY_INFO.phoneHref} className="underline hover:text-ink-900">
                {COMPANY_INFO.phoneDisplay}
              </a>
            </Detail>
            <Detail icon={MapPin} label="Location">
              {COMPANY_INFO.location}
            </Detail>
            <Detail icon={Clock} label="Opening hours">
              {COMPANY_INFO.hours}
            </Detail>
          </ul>
        </Card>

        <Card>
          <form onSubmit={handleSubmit} noValidate className="flex flex-col gap-5">
            <h2 className="text-lg font-semibold text-ink-900">Send us a message</h2>

            {sendMessage.isSuccess && (
              <Alert variant="success" title="Message sent">
                Thank you. We will reply to the email address you gave.
              </Alert>
            )}
            {sendMessage.isError && (
              <Alert variant="error">{errorMessage(sendMessage.error)}</Alert>
            )}

            <FormField label="Name" required error={errors.name}>
              <Input
                autoComplete="name"
                maxLength={NAME_MAX_CHARS}
                value={name}
                onChange={(event) => setName(event.target.value)}
                disabled={sendMessage.isPending}
                hasError={Boolean(errors.name)}
              />
            </FormField>

            <FormField label="Email address" required error={errors.email}>
              <Input
                type="email"
                autoComplete="email"
                maxLength={EMAIL_MAX_CHARS}
                value={email}
                onChange={(event) => setEmail(event.target.value)}
                disabled={sendMessage.isPending}
                hasError={Boolean(errors.email)}
              />
            </FormField>

            <FormField label="Message" required error={errors.message}>
              <TextArea
                rows={6}
                maxLength={MESSAGE_MAX_CHARS}
                value={message}
                onChange={(event) => setMessage(event.target.value)}
                disabled={sendMessage.isPending}
                hasError={Boolean(errors.message)}
              />
            </FormField>

            <Button type="submit" className="w-fit" isLoading={sendMessage.isPending}>
              Send message
            </Button>
          </form>
        </Card>
      </div>
    </div>
  );
}

export { ContactPage };
