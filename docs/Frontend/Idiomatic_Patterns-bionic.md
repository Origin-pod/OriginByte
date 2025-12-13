# Front-End Micro-Patterns for **Se**nior **Eng**ineers (LLD **Pr**ep)

**Th**is **doc**ument **co**vers micro-level front-end **de**sign **pat**terns **th**at a **se**nior **sof**tware **eng**ineer **sh**ould **kn**ow for low-level **de**sign (LLD) and **sy**stem **de**sign **int**erviews.

---

## 🎨 UI **Com**ponent **Pat**terns

- **Con**trolled vs. **Unc**ontrolled **Com**ponents\*\*
  - **Con**trolled: **St**ate **ma**naged via **pr**ops (e.g., **con**trolled `<**in**put>`).
  - **Unc**ontrolled: **St**ate **he**ld in DOM via refs/default **va**lues.
- **Con**tainer vs. **Pre**sentational **Com**ponents\*\*
  - **Con**tainer (**sm**art): **Ha**ndles **da**ta, **st**ate, side-effects.
  - **Pre**sentational (**du**mb): **Pu**rely UI **ren**dering.
- **Com**pound **Com**ponents\*\*
  - **Pa**rent **com**ponent **pro**vides **co**ntext to **ch**ild **com**ponents (`<Tabs><Tab/><TabPanel/></Tabs>`).
- **Re**nder **Pr**ops\*\*
  - **Pa**ssing a **fun**ction as a **pr**op to **co**ntrol **ren**dering.
- **Higher-Order **Com**ponents (**HO**Cs)**
  - **Wra**pping **lo**gic **ar**ound a **com**ponent (**la**rgely **rep**laced by **ho**oks).
- **Slot/Children **Pa**ttern**
  - **Pa**ssing **arb**itrary **co**ntent via `**chi**ldren` or **sl**ots (Vue/Svelte).
- **Con**trolled **Fo**rm **Han**dling\*\*
  - **Deb**ounced **in**puts, **val**idation, dirty-state **tra**cking.
- **Hea**dless **Com**ponent **Pa**ttern\*\*
  - **Lo**gic **wi**thout **st**yles; **us**ers **pr**ovide **re**nder **str**ucture.
- **Fal**lback / **Ske**letons / **Lo**ading **St**ates\*\*
- **Er**ror **Bou**ndaries\*\*
  - **Cat**ching **re**nder **er**rors and **dis**playing **fal**lback UI.

---

## 🧩 **St**ate **Man**agement Micro-Patterns

- **Lo**cal **Com**ponent **St**ate** (**ho**oks, **si**gnals, **rea**ctive **va\*\*rs).
- **De**rived **St**ate** (**co**mpute **in**stead of **st**oring **red\*\*undantly).
- **Li**fting **St**ate Up** (**sh**ared **own\*\*ership).
- **Co**ntext for Cross-Cutting **St**ate** (**gl**obal **pro\*\*vider).
- **Re**ducer **Pa**ttern** (`**use\*\*Reducer`, Redux-style).
- **St**ate **Mac**hines & **Sta**techarts** (**XS\*\*tate).
- **At**omic **St**ate\*\* (Recoil/Jotai).
- **Ev**ent **Emi**tters / Pub-Sub\*\*.
- **Imm**utable **St**ate **Up**dates\*\*.
- **Undo/Redo **St**ack** (**ti**me **tr**avel for UI).

---

## 🔄 **Da**ta **Fet**ching & **Ca**ching **Pat**terns

- **SWR / Stale-While-Revalidate**.
- **Sus**pense for **Da**ta **Fet**ching\*\*.
- **Inf**inite **Sc**roll vs. **Pag**ination\*\*.
- **Re**quest **Ded**uplication\*\*.
- **Opt**imistic UI **Up**dates\*\*.
- **Pre**fetching **Da**ta on Hover/Idle\*\*.
- **Ca**che **Inv**alidation **Str**ategies\*\*.

---

## 📦 **Ren**dering **Opt**imization **Pat**terns

- **Vir**tualized **Li**sts** (**win\*\*dowing).
- **La**zy **Lo**ading / **Co**de **Spl**itting\*\*.
- **Mem**oization** (`**us**eMemo`, `**use\*\*Callback`).
- **Pu**re **Com**ponents / `React.memo`\*\*.
- **Skeleton/Placeholder **Ren**dering**.
- **Sh**allow **Ren**dering / **Sel**ectors\*\*.
- **Con**currency **Pat**terns** (**Re**act **con**current **mo\*\*de).

---

## 🖇️ **Com**position & **Re**use Micro-Patterns

- **Cu**stom **Ho**oks / **Com**posables\*\*.
- **Dec**orator **Pa**ttern\*\*.
- **Mi**xins (**le**gacy, but **rel**evant)\*\*.
- **Ad**apter **Pa**ttern** (**nor\*\*malize props/APIs).
- **Fa**cade **Pa**ttern** (**sim**plified **AP\*\*Is).

---

## 🌍 **Acc**essibility & **Int**eraction **Pat**terns

- **Key**board **Nav**igation** (**fo**cus **tr**aps, **ar**row key **mov\*\*ement).
- **AR**IA **Ro**les & **Li**ve **Re**gions\*\*.
- **Acc**essible **Mo**dals, **Me**nus, **Too**ltips\*\*.
- **Sk**ip **Li**nks & **Fo**cus **Man**agement\*\*.
- **Cl**ick **Ou**tside / **Es**cape Key **Han**dling\*\*.
- **Res**ponsive **La**yout **Adj**ustments** (**bre**akpoints, **ada**ptive **pat\*\*terns).

---

## ⚡ **Ev**ent **Han**dling & DOM **Pat**terns

- **Del**egated **Ev**ent **Han**dling\*\*.
- **Thr**ottling & **Deb**ouncing **Ev**ents\*\*.
- **Pa**ssive **Ev**ent **Lis**teners\*\*.
- **Syn**thetic **Ev**ents vs. **Na**tive **Ev**ents\*\*.
- **Ge**sture **Han**dling** (pan, **pi**nch, **sw\*\*ipe).
- **Int**ersectionObserver for **La**zy **Ele**ments\*\*.

---

## 🛠️ **Er**ror **Han**dling & **Res**ilience **Pat**terns

- **Re**try **wi**th **Ba**ckoff** (**exp**onential, **ji\*\*tter).
- **Gra**ceful **Fal**lback UI\*\*.
- **Bou**ndary **Gu**ards** (**pr**op **val**idation, **ty**pe **nar\*\*rowing).
- **Ci**rcuit **Br**eaker **Pa**ttern** (**st**op **ret**rying **br**oken **AP\*\*Is).

---

## 🧱 **St**yling & **Th**eming **Pat**terns

- **CSS-in-JS / CSS **Mo**dules / **Ut**ility **Cl**asses**.
- **At**omic CSS / Tailwind-like **pat**terns\*\*.
- **BEM & **Sc**oped **Na**ming **Con**ventions**.
- **Th**eme **Pro**vider **Pa**ttern\*\*.
- **Res**ponsive **De**sign **To**kens\*\*.
- **De**sign **To**kens **wi**th CSS **Va**rs\*\*.
- **Dy**namic **St**yling **ba**sed on **St**ate\*\*.

---

## 🔐 Security-Oriented Micro-Patterns

- **San**itization & **Esc**aping** (XSS **pre\*\*vention).
- **St**rict CSP-Compatible **Ren**dering\*\*.
- **Cli**ckjacking **Pre**vention\*\*.
- **Se**cure **Co**okie **Han**dling\*\*.
- **JWT **St**orage **Str**ategies** (in-memory vs. **loc**alStorage).

---

## 🏗️ **Te**sting & **Deb**ugging **Pat**terns

- **Te**st ID / **Da**ta **Att**ributes for **Sel**ectors\*\*.
- **Mo**cking API **Req**uests** (MSW, **fa**ke **se\*\*rvers).
- **Sna**pshot **Te**sting\*\*.
- **E2E **Pat**terns** (**Cy**press, **Pla**ywright).
- **Vi**sual **Reg**ression **Te**sting** (**Pe**rcy, **Chr\*\*omatic).

---

## 🧮 **Alg**orithmic / **Ut**ility Micro-Patterns

- **Di**ffing **Alg**orithms** (**vi\*\*rtual DOM).
- **String/HTML **Esc**aping **Uti**lities**.
- **De**ep **Cl**one vs. **Sh**allow **Cl**one\*\*.
- **Deb**ounce & **Thr**ottle **Imp**lementations\*\*.
- **Bi**nary **Se**arch in UI** (**aut**ocomplete, **se\*\*arch).
- **Pri**ority **Qu**eues** (**ta**sk **sch\*\*eduling).

---

## ✅ Senior-Level **Int**erview **Exp**ectations

- Be **ab**le to **dem**onstrate trade-offs\*\*:
  - **Opt**imistic vs. **pes**simistic **up**dates.
  - **Pag**ination vs. **vir**tualization.
  - **Deb**ounce vs. **thr**ottle.
  - **Co**ntext vs. **li**fting **st**ate.
- **Sh**ow **sca**lability & **mai**ntainability\*\* **thi**nking.
- **Hig**hlight **per**formance, **acc**essibility, and **sec**urity\*\* **con**cerns.
