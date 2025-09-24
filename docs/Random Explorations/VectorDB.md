What are vector databases ?

So basically, vector DBs store not semantic variables or index single variables or values but rather embeddings are indexed.

What is Embeddings?

How it works

Embedding step:

You take some data — text, images, audio, even proteins.

Feed it through a model (like OpenAI’s text-embedding-3-small).

The model spits out a vector: a numerical “fingerprint of meaning.”

Example:

“cat” → [0.1, 0.9, -0.3, …]

“dog” → [0.2, 0.95, -0.25, …]

“car” → [0.88, -0.1, 0.45, …]

Notice how “cat” and “dog” are closer than “cat” and “car.”

Storage step:

You put all those vectors into a vector DB (HelixDB, Pinecone, Weaviate, Milvus).

It organizes them in a way that makes similarity search fast, even if you have millions.

Query step:

When you ask a question (say, “small house pet that purrs”), the system embeds your query into a vector.

The DB finds which stored vectors are closest in this high-dimensional space.

Closeness = semantic similarity.
