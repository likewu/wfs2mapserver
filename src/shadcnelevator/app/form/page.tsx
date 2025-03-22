import { createPost } from '@/app/form/post'
 
export default async function Page() {
    return (
    <div className="flex flex-col items-center justify-items-center min-h-screen p-8 pt-24 gap-12 font-[family-name:var(--font-geist-sans)]">
      <main className="flex-1 flex flex-col items-center justify-items-center gap-12">
        <div className="flex flex-col items-center justify-items-center gap-2">
          <h1 className="text-2xl font-medium">form</h1>
              <form action={createPost}>
                <input type="text" name="title" />
                <input type="text" name="content" />
                <button type="submit">Create</button>
              </form>
        </div>
      </main>
      <footer className="flex flex-col items-center justify-items-center gap-2">
        <div>
        </div>
      </footer>
    </div>
  );
}