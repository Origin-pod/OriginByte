import books from './goodreads.json';
import { libraryFiles } from './library-files';

export type ReadingStatus = 'reading' | 'read' | 'want-to-read';

export interface LibraryBook {
  title: string;
  author?: string;
  status: ReadingStatus;
  fileUrl?: string;
}

const statuses = new Set<ReadingStatus>(['reading', 'read', 'want-to-read']);

export const library: LibraryBook[] = (books as { title: string; author?: string; status: string }[])
  .filter((book): book is { title: string; author?: string; status: ReadingStatus } => statuses.has(book.status as ReadingStatus))
  .map((book) => ({
    title: book.title,
    author: book.author || undefined,
    status: book.status,
    fileUrl: libraryFiles[book.title] || undefined,
  }))
  .sort((a, b) => a.title.localeCompare(b.title));

/** The link from the pre-Index landing page. */
export const GOODREADS_URL = 'https://www.goodreads.com/review/list/166403141?shelf=%23ALL%23';
