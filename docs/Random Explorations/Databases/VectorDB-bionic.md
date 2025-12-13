**Wh**at are **ve**ctor **dat**abases ?

So **bas**ically, **ve**ctor DBs **st**ore not **sem**antic **var**iables or **in**dex **si**ngle **var**iables or **va**lues but **ra**ther **emb**eddings are **in**dexed.

**Wh**at is **Emb**eddings?

How it **wo**rks

**Emb**edding **st**ep:

You **ta**ke **so**me **da**ta — **te**xt, **im**ages, **au**dio, **ev**en **pro**teins.

**Fe**ed it **th**rough a **mo**del (**li**ke OpenAI’s text-embedding-3-small).

The **mo**del **sp**its out a **ve**ctor: a **num**erical “**fin**gerprint of **me**aning.”

**Ex**ample:

“cat” → [0.1, 0.9, -0.3, …]

“dog” → [0.2, 0.95, -0.25, …]

“car” → [0.88, -0.1, 0.45, …]

**No**tice how “cat” and “dog” are **cl**oser **th**an “cat” and “car.”

**St**orage **st**ep:

You put all **th**ose **ve**ctors **in**to a **ve**ctor DB (**He**lixDB, **Pin**econe, **Wea**viate, **Mi**lvus).

It **org**anizes **th**em in a way **th**at **ma**kes **sim**ilarity **se**arch **fa**st, **ev**en if you **ha**ve **mil**lions.

**Qu**ery **st**ep:

**Wh**en you ask a **que**stion (say, “**sm**all **ho**use pet **th**at **pu**rrs”), the **sy**stem **em**beds **yo**ur **qu**ery **in**to a **ve**ctor.

The DB **fi**nds **wh**ich **st**ored **ve**ctors are **cl**osest in **th**is high-dimensional **sp**ace.

**Clo**seness = **sem**antic **sim**ilarity.
