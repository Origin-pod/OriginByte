# **Dat**abase **Sy**stem **Dia**grams

**Th**is **doc**ument **con**tains **Me**rmaid **dia**grams **th**at **vis**ualize key **dat**abase **con**cepts and **arc**hitectures.

## **Dat**abase **Ty**pes **Cla**ssification

```**me**rmaid
**gr**aph TD
    A[Database **Sy**stems] --> B[Relational **RD**BMS]
    A --> C[NoSQL **Dat**abases]
    A --> D[NewSQL **Dat**abases]
    A --> E[Specialized **Dat**abases]

    B --> B1[MySQL]
    B --> B2[PostgreSQL]
    B --> B3[Oracle]
    B --> B4[SQL **Se**rver]

    C --> C1[Document **Mo**ngoDB]
    C --> C2[Key-Value **Re**dis]
    C --> C3[Column **Cas**sandra]
    C --> C4[Graph **Ne**o4j]

    D --> D1[CockroachDB]
    D --> D2[Google **Sp**anner]
    D --> D3[TiDB]

    E --> E1[Time-Series **Inf**luxDB]
    E --> E2[In-Memory **Re**dis]
    E --> E3[Search **Ela**sticsearch]
```

## **Dat**abase **Sy**stem **Arc**hitecture

```**me**rmaid
**gr**aph TB
    **sub**graph "**Cl**ient **La**yer"
        A[Web **App**lication]
        B[Mobile App]
        C[Desktop App]
    end

    **sub**graph "**App**lication **La**yer"
        D[API **Ga**teway]
        E[Business **Lo**gic]
        F[Authentication]
    end

    **sub**graph "**Dat**abase **La**yer"
        G[Query **Pro**cessor]
        H[Storage **Ma**nager]
        I[Transaction **Ma**nager]
        J[Catalog **Ma**nager]
    end

    **sub**graph "**St**orage **La**yer"
        K[Primary **St**orage]
        L[Backup **St**orage]
        M[Cache **La**yer]
    end

    A --> D
    B --> D
    C --> D
    D --> E
    E --> F
    E --> G
    G --> H
    H --> I
    I --> J
    H --> K
    K --> L
    H --> M
```

## **AC**ID **Pro**perties **Vis**ualization

```**me**rmaid
**gr**aph LR
    **sub**graph "**Tra**nsaction **Lif**ecycle"
        A[Begin **Tra**nsaction] --> B[Execute **Ope**rations]
        B --> C{All Operations<br/>Successful?}
        C -->|Yes| D[Commit]
        C -->|No| E[Rollback]
        D --> F[ACID Properties<br/>Ensured]
        E --> G[Database State<br/>Restored]
    end

    **sub**graph "**AC**ID **Pro**perties"
        H[Atomicity<br/>All or **No**thing]
        I[Consistency<br/>Valid **St**ate]
        J[Isolation<br/>Concurrent **Sa**fety]
        K[Durability<br/>Permanent **St**orage]
    end

    F --> H
    F --> I
    F --> J
    F --> K
```

## **Dat**abase **Nor**malization **Pr**ocess

```**me**rmaid
**gr**aph TD
    A[Unnormalized **Da**ta] --> B[1NF<br/>Atomic **Va**lues]
    B --> C[2NF<br/>No **Pa**rtial **Dep**endencies]
    C --> D[3NF<br/>No **Tra**nsitive **Dep**endencies]
    D --> E[BCNF<br/>Every **Det**erminant is Key]
    E --> F[4NF<br/>No **Mul**tivalued **Dep**endencies]
    F --> G[5NF<br/>No **Jo**in **Dep**endencies]

    A --> A1[Raw **Da**ta with<br/>Repeating **Gr**oups]
    B --> B1[Each Column<br/>Contains **Si**ngle **Va**lue]
    C --> C1[All Non-Key Attributes<br/>Depend on **Fu**ll Key]
    D --> D1[No Non-Key Attribute<br/>Depends on Non-Key]
    E --> E1[Every Determinant<br/>is **Can**didate Key]
```

## **Dat**abase **In**dex **Ty**pes

```**me**rmaid
**gr**aph TD
    A[Database **In**dexes] --> B[B-Tree **In**dex]
    A --> C[Hash **In**dex]
    A --> D[Bitmap **In**dex]
    A --> E[Full-Text **In**dex]
    A --> F[Composite **In**dex]

    B --> B1[Range **Qu**eries]
    B --> B2[Equality **Sea**rches]
    B --> B3[Sorted **Re**sults]

    C --> C1[Fast **Equ**ality]
    C --> C2[No **Ra**nge **Su**pport]
    C --> C3[Fixed **Ha**sh **Si**ze]

    D --> D1[Low **Car**dinality]
    D --> D2[Data **War**ehousing]
    D --> D3[Space **Eff**icient]

    E --> E1[Text **Se**arch]
    E --> E2[Full-Text **Qu**eries]
    E --> E3[Relevance **Sc**oring]

    F --> F1[Multiple **Co**lumns]
    F --> F2[Query **Opt**imization]
    F --> F3[Covering **In**dexes]
```

## **Dat**abase **Rep**lication **Ty**pes

```**me**rmaid
**gr**aph TB
    **sub**graph "Master-Slave **Rep**lication"
        A[Master **Dat**abase] --> B[Slave 1]
        A --> C[Slave 2]
        A --> D[Slave 3]
        B --> B1[Read **Ope**rations]
        C --> C1[Read **Ope**rations]
        D --> D1[Read **Ope**rations]
    end

    **sub**graph "Master-Master **Rep**lication"
        E[Master 1] <--> F[Master 2]
        E --> E1[Read/Write]
        F --> F1[Read/Write]
    end

    **sub**graph "Multi-Master **Rep**lication"
        G[Master 1] <--> H[Master 2]
        G <--> I[Master 3]
        H <--> I
        G --> G1[Distributed **Wr**ites]
        H --> H1[Distributed **Wr**ites]
        I --> I1[Distributed **Wr**ites]
    end
```

## **Dat**abase **Sec**urity **La**yers

```**me**rmaid
**gr**aph TB
    **sub**graph "**App**lication **Sec**urity"
        A[Authentication]
        B[Authorization]
        C[Session **Man**agement]
    end

    **sub**graph "**Dat**abase **Sec**urity"
        D[User **Man**agement]
        E[Access **Co**ntrol]
        F[Audit **Lo**gging]
    end

    **sub**graph "**Da**ta **Pro**tection"
        G[Encryption at **Re**st]
        H[Encryption in **Tr**ansit]
        I[Data **Ma**sking]
    end

    **sub**graph "**Inf**rastructure **Sec**urity"
        J[Network **Sec**urity]
        K[Firewall **Ru**les]
        L[SSL/TLS]
    end

    A --> D
    B --> E
    C --> F
    D --> G
    E --> H
    F --> I
    G --> J
    H --> K
    I --> L
```

## **Dat**abase **Per**formance **Opt**imization

```**me**rmaid
**gr**aph TD
    A[Database **Per**formance] --> B[Query **Opt**imization]
    A --> C[Index **Opt**imization]
    A --> D[Caching **Str**ategy]
    A --> E[Hardware **Opt**imization]

    B --> B1[Query **Rew**riting]
    B --> B2[Execution **Pl**ans]
    B --> B3[Statistics **Up**dates]

    C --> C1[Index **Sel**ection]
    C --> C2[Index **Mai**ntenance]
    C --> C3[Covering **In**dexes]

    D --> D1[Application **Ca**che]
    D --> D2[Database **Ca**che]
    D --> D3[Query **Re**sult **Ca**che]

    E --> E1[CPU **Opt**imization]
    E --> E2[Memory **Tu**ning]
    E --> E3[Storage I/O]
    E --> E4[Network **Con**figuration]
```

## **Mo**dern **Dat**abase **Tr**ends

```**me**rmaid
**gr**aph TB
    **sub**graph "**Cl**oud **Dat**abases"
        A[Database as a **Se**rvice]
        B[Serverless **Dat**abases]
        C[Managed **Ser**vices]
    end

    **sub**graph "**Dis**tributed **Sy**stems"
        D[Sharding]
        E[Partitioning]
        F[Replication]
    end

    **sub**graph "New **Par**adigms"
        G[NewSQL]
        H[Multi-Model]
        I[Graph **Dat**abases]
    end

    **sub**graph "**Aut**omation"
        J[Infrastructure as **Co**de]
        K[Database **De**vOps]
        L[Auto-Scaling]
    end

    A --> D
    B --> E
    C --> F
    D --> G
    E --> H
    F --> I
    G --> J
    H --> K
    I --> L
```

## **Dat**abase **Tra**nsaction **St**ates

```**me**rmaid
stateDiagram-v2
    [*] --> **Ac**tive
    **Ac**tive --> **Par**tiallyCommitted : **Ex**ecute **Ope**rations
    **Par**tiallyCommitted --> **Com**mitted : **Co**mmit
    **Par**tiallyCommitted --> **Fa**iled : **Er**ror **Oc**curs
    **Fa**iled --> **Ab**orted : **Rol**lback
    **Ab**orted --> [*]
    **Com**mitted --> [*]

    **no**te **ri**ght of **Ac**tive : **Tra**nsaction **st**arts
    **no**te **ri**ght of **Par**tiallyCommitted : All **ope**rations **com**plete
    **no**te **ri**ght of **Com**mitted : **Ch**anges **ma**de **per**manent
    **no**te **ri**ght of **Fa**iled : **Er**ror **det**ected
    **no**te **ri**ght of **Ab**orted : **Ch**anges **un**done
```

## **Dat**abase **Sc**hema **Evo**lution

```**me**rmaid
**gr**aph LR
    A[Version 1.0] --> B[Add **Co**lumn]
    B --> C[Version 1.1]
    C --> D[Modify **Co**lumn **Ty**pe]
    D --> E[Version 1.2]
    E --> F[Add **In**dex]
    F --> G[Version 1.3]
    G --> H[Drop **Co**lumn]
    H --> I[Version 2.0]

    A --> A1[Initial **Sc**hema]
    C --> C1[Backward **Com**patible]
    E --> E1[Breaking **Ch**ange]
    G --> G1[Performance **Imp**rovement]
    I --> I1[Major **Ve**rsion]
```

## **Dat**abase **Ba**ckup and **Rec**overy

```**me**rmaid
**gr**aph TD
    A[Database **Ba**ckup **Str**ategy] --> B[Full **Ba**ckup]
    A --> C[Incremental **Ba**ckup]
    A --> D[Differential **Ba**ckup]
    A --> E[Transaction Log **Ba**ckup]

    B --> B1[Complete **Dat**abase **Co**py]
    B --> B2[Weekly **Sch**edule]

    C --> C1[Changes **Si**nce **La**st **Ba**ckup]
    C --> C2[Daily **Sch**edule]

    D --> D1[Changes **Si**nce **Fu**ll **Ba**ckup]
    D --> D2[Daily **Sch**edule]

    E --> E1[Transaction Log **Re**cords]
    E --> E2[Point-in-Time **Rec**overy]

    F[Recovery **Pr**ocess] --> G[Restore **Fu**ll **Ba**ckup]
    G --> H[Apply **Tra**nsaction **Lo**gs]
    H --> I[Database **Res**tored]
```

## **Dat**abase **Mon**itoring and **Ale**rting

```**me**rmaid
**gr**aph TB
    **sub**graph "**Mon**itoring **Me**trics"
        A[Performance **Me**trics]
        B[Resource **Us**age]
        C[Error **Ra**tes]
        D[Query **Per**formance]
    end

    **sub**graph "**Ale**rting **Sy**stem"
        E[Threshold **Mon**itoring]
        F[Anomaly **Det**ection]
        G[Automated **Res**ponses]
    end

    **sub**graph "**Rep**orting"
        H[Dashboard **Vi**ews]
        I[Performance **Re**ports]
        J[Capacity **Pla**nning]
    end

    A --> E
    B --> F
    C --> G
    D --> H
    E --> I
    F --> J
    G --> H
```

**Th**ese **dia**grams **pr**ovide **vi**sual **rep**resentations of key **dat**abase **con**cepts, **ma**king it **ea**sier to **und**erstand the **rel**ationships and **pro**cesses **inv**olved in **dat**abase **sy**stems.
