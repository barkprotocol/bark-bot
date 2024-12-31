import Image from "next/image";

async function sendTelegramMessage(message: string) {
  const TELEGRAM_API_URL = `https://api.telegram.org/bot${process.env.NEXT_PUBLIC_TELEGRAM_BOT_TOKEN}/sendMessage`;
  const CHAT_ID = process.env.NEXT_PUBLIC_TELEGRAM_CHAT_ID;

  try {
    const response = await fetch(`${TELEGRAM_API_URL}?chat_id=${CHAT_ID}&text=${encodeURIComponent(message)}`);
    const data = await response.json();

    console.log('Telegram API response:', data);

    if (data.ok) {
      console.log('Message sent successfully');
    } else {
      console.error('Error sending message:', data);
    }
  } catch (error) {
    console.error('Failed to send message:', error);
  }
}

export default function Home() {
  return (
    <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
        <h1 className="text-xl sm:text-2xl font-semibold text-center sm:text-left">
          Welcome to BARK Bot
        </h1>
        <p className="text-center sm:text-left text-sm text-gray-600 font-[family-name:var(--font-geist-mono)] mb-4">
          BARK Bot helps you stay connected and receive instant notifications in your Telegram. 
          Get real-time updates, news, and events directly through Telegram messages.
        </p>
        <ol className="list-inside list-decimal text-sm text-center sm:text-left font-[family-name:var(--font-geist-mono)]">
          <li className="mb-2">
            BARK BOT{" "}
            <code className="bg-black/[.05] dark:bg-white/[.06] px-1 py-0.5 rounded font-semibold">
              Notification Service
            </code>
          </li>
          <li>Get instant updates on important events and actions.</li>
          <li>Stay connected with real-time notifications in Telegram.</li>
          <li>Save and track your changes instantly through the bot.</li>
        </ol>

        <div className="flex gap-4 items-center flex-col sm:flex-row">
          <a
            className="rounded-full border border-solid border-transparent transition-colors flex items-center justify-center bg-foreground text-background gap-2 hover:bg-[#383838] dark:hover:bg-[#ccc] text-sm sm:text-base h-10 sm:h-12 px-4 sm:px-5"
            href="https://t.me/BARKBot" // Replace with your actual bot URL
            target="_blank"
            rel="noopener noreferrer"
          >
            <Image
              className="dark:invert"
              src="https://ucarecdn.com/bbc74eca-8e0d-4147-8a66-6589a55ae8d0/bark.webp"
              alt="BARK logomark"
              width={20}
              height={20}
            />
            Start Bot
          </a>
        </div>
      </main>

      <footer className="row-start-3 flex gap-6 flex-wrap items-center justify-center">
        <a
          className="flex items-center gap-2 hover:underline hover:underline-offset-4"
          href="https://nextjs.org/docs?utm_source=create-next-app&utm_medium=appdir-template-tw&utm_campaign=create-next-app"
          target="_blank"
          rel="noopener noreferrer"
        >
          <Image
            aria-hidden
            src="/file.svg"
            alt="File icon"
            width={16}
            height={16}
          />
          Read Docs
        </a>
      </footer>
    </div>
  );
}
