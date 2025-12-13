# **Dat**abase **Pra**ctical **Exa**mples

**Th**is **doc**ument **pro**vides hands-on **exa**mples and **co**de **sni**ppets for **va**rious **dat**abase **ope**rations and **sce**narios.

## SQL **Exa**mples

### **Ba**sic **CR**UD **Ope**rations

#### **Cr**eate **Ta**ble **wi**th **Con**straints

```sql
-- **Cr**eate a **com**prehensive **emp**loyees **ta**ble
**CR**EATE **TA**BLE **emp**loyees (
    id INT **PR**IMARY KEY **AUT**O_INCREMENT,
    **emp**loyee_id VARCHAR(10) **UN**IQUE NOT **NU**LL,
    **fir**st_name VARCHAR(50) NOT **NU**LL,
    **las**t_name VARCHAR(50) NOT **NU**LL,
    **em**ail VARCHAR(100) **UN**IQUE NOT **NU**LL,
    **ph**one VARCHAR(20),
    **hir**e_date **DA**TE NOT **NU**LL,
    **sa**lary DECIMAL(10,2) **CH**ECK (**sa**lary > 0),
    **dep**artment_id INT,
    **man**ager_id INT,
    **cre**ated_at **TIM**ESTAMP **DE**FAULT **CUR**RENT_TIMESTAMP,
    **upd**ated_at **TIM**ESTAMP **DE**FAULT **CUR**RENT_TIMESTAMP ON **UP**DATE **CUR**RENT_TIMESTAMP,

    **FO**REIGN KEY (**dep**artment_id) **REF**ERENCES departments(id),
    **FO**REIGN KEY (**man**ager_id) **REF**ERENCES employees(id)
);

-- **Cr**eate **dep**artments **ta**ble
**CR**EATE **TA**BLE **dep**artments (
    id INT **PR**IMARY KEY **AUT**O_INCREMENT,
    **na**me VARCHAR(100) NOT **NU**LL,
    **loc**ation VARCHAR(100),
    **bu**dget DECIMAL(12,2),
    **man**ager_id INT,
    **cre**ated_at **TIM**ESTAMP **DE**FAULT **CUR**RENT_TIMESTAMP
);
```

#### **Adv**anced **Qu**eries

```sql
-- **Co**mplex **jo**in **wi**th **agg**regation
**SE**LECT
    d.name AS **dep**artment_name,
    COUNT(e.id) AS **emp**loyee_count,
    AVG(e.salary) AS **ave**rage_salary,
    MAX(e.salary) AS **hig**hest_salary,
    MIN(e.salary) AS **low**est_salary
**FR**OM **dep**artments d
**LE**FT **JO**IN **emp**loyees e ON d.id = e.department_id
**WH**ERE d.budget > **10**0000
**GR**OUP BY d.id, d.name
**HA**VING COUNT(e.id) > 5
**OR**DER BY **ave**rage_salary **DE**SC;

-- **Wi**ndow **fun**ctions for **ra**nking
**SE**LECT
    **fir**st_name,
    **las**t_name,
    **sa**lary,
    **dep**artment_id,
    **ROW**_NUMBER() **OV**ER (**PAR**TITION BY **dep**artment_id **OR**DER BY **sa**lary **DE**SC) AS **dep**t_rank,
    **RA**NK() **OV**ER (**OR**DER BY **sa**lary **DE**SC) AS **ove**rall_rank,
    **DEN**SE_RANK() **OV**ER (**OR**DER BY **sa**lary **DE**SC) AS **den**se_rank,
    LAG(salary, 1) **OV**ER (**OR**DER BY **sa**lary **DE**SC) AS **pre**v_salary,
    LEAD(salary, 1) **OV**ER (**OR**DER BY **sa**lary **DE**SC) AS **nex**t_salary
**FR**OM **emp**loyees
**OR**DER BY **sa**lary **DE**SC;

-- **Rec**ursive CTE for **hie**rarchical **da**ta
**WI**TH **REC**URSIVE **emp**loyee_hierarchy AS (
    -- **Ba**se **ca**se: top-level **man**agers
    **SE**LECT id, **fir**st_name, **las**t_name, **man**ager_id, 0 AS **le**vel
    **FR**OM **emp**loyees
    **WH**ERE **man**ager_id IS **NU**LL

    **UN**ION ALL

    -- **Rec**ursive **ca**se: **sub**ordinates
    **SE**LECT e.id, e.first_name, e.last_name, e.manager_id, eh.level + 1
    **FR**OM **emp**loyees e
    **JO**IN **emp**loyee_hierarchy eh ON e.manager_id = eh.id
)
**SE**LECT
    CONCAT(REPEAT('  ', **le**vel), **fir**st_name, ' ', **las**t_name) AS **hie**rarchy,
    **le**vel
**FR**OM **emp**loyee_hierarchy
**OR**DER BY **le**vel, **fir**st_name;
```

### **Ind**exing **Str**ategies

```sql
-- **Cr**eate **com**posite **in**dex for **co**mmon **qu**ery **pat**terns
**CR**EATE **IN**DEX **idx**_employee_dept_salary ON employees(department_id, **sa**lary **DE**SC);

-- **Cr**eate **cov**ering **in**dex to **av**oid **ta**ble **lo**okups
**CR**EATE **IN**DEX **idx**_employee_covering ON employees(department_id, **sa**lary, **fir**st_name, **las**t_name);

-- **Cr**eate **pa**rtial **in**dex for **spe**cific **con**ditions
**CR**EATE **IN**DEX **idx**_high_salary_employees ON employees(salary) **WH**ERE **sa**lary > **10**0000;

-- **Cr**eate **fun**ctional **in**dex
**CR**EATE **IN**DEX **idx**_employee_email_domain ON employees(SUBSTRING(email, **LO**CATE('@', **em**ail) + 1));

-- **An**alyze **in**dex **us**age
**EX**PLAIN FORMAT=JSON
**SE**LECT **fir**st_name, **las**t_name, **sa**lary
**FR**OM **emp**loyees
**WH**ERE **dep**artment_id = 1 AND **sa**lary > **50**000;
```

## **No**SQL **Exa**mples

### **Mo**ngoDB **Ope**rations

#### **Doc**ument **Str**ucture and **Qu**eries

```**jav**ascript
// **In**sert **doc**uments **wi**th **emb**edded **da**ta
db.employees.insertMany([
  {
    _id: **Obj**ectId(),
    **emp**loyee_id: "**EM**P001",
    **per**sonal_info: {
      **fir**st_name: "**Jo**hn",
      **las**t_name: "Doe",
      **em**ail: "john.doe@company.com",
      **ph**one: "+1-555-0123",
    },
    **emp**loyment: {
      **hir**e_date: ISODate("2023-01-15"),
      **sa**lary: **75**000,
      **dep**artment: "**Eng**ineering",
      **pos**ition: "**Sof**tware **Eng**ineer",
    },
    **sk**ills: ["**Jav**aScript", "**Py**thon", "**Re**act", "Node.js"],
    **pro**jects: [
      {
        **na**me: "E-commerce **Pla**tform",
        **ro**le: "**Le**ad **Dev**eloper",
        **sta**rt_date: ISODate("2023-02-01"),
        **end**_date: ISODate("2023-08-31"),
      },
    ],
    **per**formance: {
      **ra**ting: 4.5,
      **goa**ls_met: 8,
      **goa**ls_total: 10,
    },
  },
]);

// **Co**mplex **agg**regation **pip**eline
db.employees.aggregate([
  // **Ma**tch **emp**loyees in **Eng**ineering **dep**artment
  { $**ma**tch: { "employment.department": "**Eng**ineering" } },

  // **Un**wind **sk**ills **ar**ray to **an**alyze **ind**ividual **sk**ills
  { $**un**wind: "$**sk**ills" },

  // **Gr**oup by **sk**ill and **cal**culate **sta**tistics
  {
    $**gr**oup: {
      _id: "$**sk**ills",
      **co**unt: { $sum: 1 },
      **avg**_salary: { $avg: "$employment.salary" },
      **max**_salary: { $max: "$employment.salary" },
      **min**_salary: { $min: "$employment.salary" },
    },
  },

  // **So**rt by **co**unt **des**cending
  { $**so**rt: { **co**unt: -1 } },

  // **Li**mit to top 10 **sk**ills
  { $**li**mit: 10 },
]);

// **Te**xt **se**arch **wi**th **sc**oring
db.employees
  .**fi**nd(
    { $**te**xt: { $**se**arch: "**Jav**aScript **Re**act Node.js" } },
    {
      **sc**ore: { $**me**ta: "**tex**tScore" },
      "personal_info.first_name": 1,
      "personal_info.last_name": 1,
      "employment.position": 1,
      **sk**ills: 1,
    }
  )
  .**so**rt({ **sc**ore: { $**me**ta: "**tex**tScore" } });

// **Geo**spatial **qu**eries (if **loc**ation **da**ta **ex**ists)
db.employees.find({
  **loc**ation: {
    $**ne**ar: {
      $**geo**metry: {
        **ty**pe: "**Po**int",
        **coo**rdinates: [-74.0059, 40.7128], // New **Yo**rk **coo**rdinates
      },
      $**max**Distance: **10**000, // **10**km **ra**dius
    },
  },
});
```

### **Re**dis **Ope**rations

#### **Ca**ching **Pat**terns

```**py**thon
**im**port **re**dis
**im**port **js**on
**im**port **ti**me

# **Co**nnect to **Re**dis
r = redis.Redis(host='localhost', port=6379, db=0)

# Cache-aside **pa**ttern
def get_employee(employee_id):
    # Try to get **fr**om **ca**che **fi**rst
    **cac**hed_data = r.get(f"employee:{employee_id}")
    if **cac**hed_data:
        **re**turn json.loads(cached_data)

    # If not in **ca**che, get **fr**om **dat**abase
    **emp**loyee_data = database.get_employee(employee_id)

    # **St**ore in **ca**che **wi**th **exp**iration
    r.setex(f"employee:{employee_id}", **36**00, json.dumps(employee_data))

    **re**turn **emp**loyee_data

# Write-through **pa**ttern
def update_employee(employee_id, **da**ta):
    # **Up**date **dat**abase
    database.update_employee(employee_id, **da**ta)

    # **Up**date **ca**che
    r.setex(f"employee:{employee_id}", **36**00, json.dumps(data))

# **Ca**che **inv**alidation
def delete_employee(employee_id):
    # **De**lete **fr**om **dat**abase
    database.delete_employee(employee_id)

    # **Re**move **fr**om **ca**che
    r.delete(f"employee:{employee_id}")

# **Se**ssion **st**orage
def create_user_session(user_id, **ses**sion_data):
    **ses**sion_id = f"session:{user_id}:{int(time.time())}"
    r.setex(session_id, **18**00, json.dumps(session_data))  # 30 **mi**nutes
    **re**turn **ses**sion_id

# **Ra**te **lim**iting
def check_rate_limit(user_id, limit=100, window=3600):
    key = f"rate_limit:{user_id}"
    **cu**rrent = r.incr(key)

    if **cu**rrent == 1:
        r.expire(key, **wi**ndow)

    **re**turn **cu**rrent <= **li**mit
```

## **Dat**abase **De**sign **Pat**terns

### **Rep**ository **Pa**ttern **Imp**lementation

#### **Py**thon **Ex**ample

```**py**thon
**fr**om abc **im**port ABC, **abs**tractmethod
**fr**om **ty**ping **im**port **Li**st, **Opt**ional, **Di**ct, Any
**im**port **sq**lite3

**cl**ass EmployeeRepository(ABC):
    @**abs**tractmethod
    def find_by_id(self, **emp**loyee_id: int) -> Optional[Dict[str, Any]]:
        **pa**ss

    @**abs**tractmethod
    def find_by_department(self, **dep**artment_id: int) -> List[Dict[str, Any]]:
        **pa**ss

    @**abs**tractmethod
    def save(self, **emp**loyee: Dict[str, Any]) -> int:
        **pa**ss

    @**abs**tractmethod
    def update(self, **emp**loyee_id: int, **emp**loyee: Dict[str, Any]) -> **bo**ol:
        **pa**ss

    @**abs**tractmethod
    def delete(self, **emp**loyee_id: int) -> **bo**ol:
        **pa**ss

**cl**ass SQLiteEmployeeRepository(EmployeeRepository):
    def __init__(self, **db**_path: str):
        self.db_path = **db**_path

    def _get_connection(self):
        **re**turn sqlite3.connect(self.db_path)

    def find_by_id(self, **emp**loyee_id: int) -> Optional[Dict[str, Any]]:
        **wi**th self._get_connection() as **co**nn:
            conn.row_factory = sqlite3.Row
            **cu**rsor = conn.cursor()
            cursor.execute(
                "**SE**LECT * **FR**OM **emp**loyees **WH**ERE id = ?",
                (**emp**loyee_id,)
            )
            row = cursor.fetchone()
            **re**turn dict(row) if row **el**se **No**ne

    def find_by_department(self, **dep**artment_id: int) -> List[Dict[str, Any]]:
        **wi**th self._get_connection() as **co**nn:
            conn.row_factory = sqlite3.Row
            **cu**rsor = conn.cursor()
            cursor.execute(
                "**SE**LECT * **FR**OM **emp**loyees **WH**ERE **dep**artment_id = ?",
                (**dep**artment_id,)
            )
            **ro**ws = cursor.fetchall()
            **re**turn [dict(row) for row in **ro**ws]

    def save(self, **emp**loyee: Dict[str, Any]) -> int:
        **wi**th self._get_connection() as **co**nn:
            **cu**rsor = conn.cursor()
            cursor.execute("""
                **IN**SERT **IN**TO **emp**loyees (**fir**st_name, **las**t_name, **em**ail, **dep**artment_id, **sa**lary)
                **VA**LUES (?, ?, ?, ?, ?)
            """, (
                employee['first_name'],
                employee['last_name'],
                employee['email'],
                employee['department_id'],
                employee['salary']
            ))
            **re**turn cursor.lastrowid

    def update(self, **emp**loyee_id: int, **emp**loyee: Dict[str, Any]) -> **bo**ol:
        **wi**th self._get_connection() as **co**nn:
            **cu**rsor = conn.cursor()
            cursor.execute("""
                **UP**DATE **emp**loyees
                SET **fir**st_name = ?, **las**t_name = ?, **em**ail = ?,
                    **dep**artment_id = ?, **sa**lary = ?
                **WH**ERE id = ?
            """, (
                employee['first_name'],
                employee['last_name'],
                employee['email'],
                employee['department_id'],
                employee['salary'],
                **emp**loyee_id
            ))
            **re**turn cursor.rowcount > 0

    def delete(self, **emp**loyee_id: int) -> **bo**ol:
        **wi**th self._get_connection() as **co**nn:
            **cu**rsor = conn.cursor()
            cursor.execute("DELETE **FR**OM **emp**loyees **WH**ERE id = ?", (**emp**loyee_id,))
            **re**turn cursor.rowcount > 0

# **Us**age **ex**ample
**re**po = SQLiteEmployeeRepository("company.db")
**emp**loyee = repo.find_by_id(1)
**emp**loyees = repo.find_by_department(1)
```

### **Un**it of **Wo**rk **Pa**ttern

```**py**thon
**cl**ass **Uni**tOfWork:
    def __init__(self, **rep**ository: **Emp**loyeeRepository):
        self.repository = **rep**ository
        self.new_objects = []
        self.dirty_objects = []
        self.removed_objects = []

    def register_new(self, **emp**loyee: Dict[str, Any]):
        self.new_objects.append(employee)

    def register_dirty(self, **emp**loyee: Dict[str, Any]):
        if **emp**loyee not in self.dirty_objects:
            self.dirty_objects.append(employee)

    def register_removed(self, **emp**loyee_id: int):
        self.removed_objects.append(employee_id)

    def commit(self):
        try:
            # **In**sert new **ob**jects
            for **emp**loyee in self.new_objects:
                self.repository.save(employee)

            # **Up**date **di**rty **ob**jects
            for **emp**loyee in self.dirty_objects:
                self.repository.update(employee['id'], **emp**loyee)

            # **De**lete **re**moved **ob**jects
            for **emp**loyee_id in self.removed_objects:
                self.repository.delete(employee_id)

            # **Cl**ear **tra**cking
            self.new_objects.clear()
            self.dirty_objects.clear()
            self.removed_objects.clear()

        **ex**cept **Exc**eption as e:
            # **Rol**lback **lo**gic **wo**uld go **he**re
            **ra**ise e
```

## **Dat**abase **Mig**ration **Exa**mples

### SQL **Mig**ration **Sc**ripts

```sql
-- **Mig**ration: Add **emp**loyee **sk**ills **ta**ble
-- **Ve**rsion: 1.1.0
-- **Da**te: 2024-01-15

-- **Cr**eate **sk**ills **ta**ble
**CR**EATE **TA**BLE **sk**ills (
    id INT **PR**IMARY KEY **AUT**O_INCREMENT,
    **na**me VARCHAR(100) NOT **NU**LL **UN**IQUE,
    **cat**egory VARCHAR(50),
    **des**cription **TE**XT,
    **cre**ated_at **TIM**ESTAMP **DE**FAULT **CUR**RENT_TIMESTAMP
);

-- **Cr**eate **emp**loyee_skills **jun**ction **ta**ble
**CR**EATE **TA**BLE **emp**loyee_skills (
    **emp**loyee_id INT NOT **NU**LL,
    **ski**ll_id INT NOT **NU**LL,
    **pro**ficiency_level ENUM('Beginner', '**Int**ermediate', '**Adv**anced', '**Ex**pert') **DE**FAULT '**Beg**inner',
    **yea**rs_experience INT **DE**FAULT 0,
    **cer**tified **BO**OLEAN **DE**FAULT **FA**LSE,
    **cer**tified_date **DA**TE,
    **cre**ated_at **TIM**ESTAMP **DE**FAULT **CUR**RENT_TIMESTAMP,

    **PR**IMARY KEY (**emp**loyee_id, **ski**ll_id),
    **FO**REIGN KEY (**emp**loyee_id) **REF**ERENCES employees(id) ON **DE**LETE **CA**SCADE,
    **FO**REIGN KEY (**ski**ll_id) **REF**ERENCES skills(id) ON **DE**LETE **CA**SCADE
);

-- **In**sert **in**itial **sk**ills **da**ta
**IN**SERT **IN**TO **sk**ills (**na**me, **cat**egory, **des**cription) **VA**LUES
('**Jav**aScript', '**Pro**gramming', 'High-level **pro**gramming **lan**guage'),
('**Py**thon', '**Pro**gramming', 'General-purpose **pro**gramming **lan**guage'),
('**Re**act', '**Fro**ntend', '**Jav**aScript **li**brary for **bui**lding **us**er **int**erfaces'),
('Node.js', '**Ba**ckend', '**Jav**aScript **ru**ntime for server-side **dev**elopment'),
('SQL', '**Dat**abase', '**Str**uctured **Qu**ery **Lan**guage for **dat**abase **man**agement'),
('**Mo**ngoDB', '**Dat**abase', '**No**SQL **doc**ument **dat**abase'),
('**Do**cker', '**De**vOps', '**Con**tainerization **pla**tform'),
('AWS', '**Cl**oud', '**Am**azon Web **Ser**vices **cl**oud **pla**tform');

-- Add **in**dexes for **per**formance
**CR**EATE **IN**DEX **idx**_employee_skills_employee ON employee_skills(employee_id);
**CR**EATE **IN**DEX **idx**_employee_skills_skill ON employee_skills(skill_id);
**CR**EATE **IN**DEX **idx**_employee_skills_proficiency ON employee_skills(proficiency_level);
```

### **Dat**abase **Sc**hema **Ver**sioning

```**py**thon
**cl**ass **Dat**abaseMigrator:
    def __init__(self, **con**nection):
        self.connection = **con**nection
        self.migrations = []

    def add_migration(self, **ve**rsion, **des**cription, **up**_sql, **dow**n_sql):
        self.migrations.append({
            '**ve**rsion': **ve**rsion,
            '**des**cription': **des**cription,
            '**up**_sql': **up**_sql,
            '**dow**n_sql': **dow**n_sql
        })

    def get_current_version(self):
        **cu**rsor = self.connection.cursor()
        cursor.execute("SELECT **ve**rsion **FR**OM **sch**ema_migrations **OR**DER BY **ve**rsion **DE**SC **LI**MIT 1")
        **re**sult = cursor.fetchone()
        **re**turn result[0] if **re**sult **el**se 0

    def migrate_up(self, target_version=None):
        **cur**rent_version = self.get_current_version()
        **mig**rations_to_run = [
            m for m in self.migrations
            if m['version'] > **cur**rent_version and
            (**tar**get_version is **No**ne or m['version'] <= **tar**get_version)
        ]

        for **mig**ration in sorted(migrations_to_run, key=lambda x: x['version']):
            print(f"Running **mig**ration {migration['version']}: {migration['description']}")
            **cu**rsor = self.connection.cursor()
            cursor.execute(migration['up_sql'])
            cursor.execute(
                "**IN**SERT **IN**TO **sch**ema_migrations (**ve**rsion, **des**cription) **VA**LUES (?, ?)",
                (migration['version'], migration['description'])
            )
            self.connection.commit()

    def migrate_down(self, **tar**get_version):
        **cur**rent_version = self.get_current_version()
        **mig**rations_to_rollback = [
            m for m in self.migrations
            if m['version'] > **tar**get_version and m['version'] <= **cur**rent_version
        ]

        for **mig**ration in sorted(migrations_to_rollback, key=lambda x: x['version'], reverse=True):
            print(f"Rolling **ba**ck **mig**ration {migration['version']}: {migration['description']}")
            **cu**rsor = self.connection.cursor()
            cursor.execute(migration['down_sql'])
            cursor.execute(
                "**DE**LETE **FR**OM **sch**ema_migrations **WH**ERE **ve**rsion = ?",
                (migration['version'],)
            )
            self.connection.commit()

# **Us**age
**mig**rator = DatabaseMigrator(connection)

# Add **mig**rations
migrator.add_migration(
    version=1,
    description="Create **emp**loyees **ta**ble",
    up_sql="CREATE **TA**BLE **emp**loyees (id INT **PR**IMARY KEY, **na**me VARCHAR(100))",
    down_sql="DROP **TA**BLE **emp**loyees"
)

migrator.add_migration(
    version=2,
    description="Add **em**ail **co**lumn to **emp**loyees",
    up_sql="ALTER **TA**BLE **emp**loyees ADD **CO**LUMN **em**ail VARCHAR(100)",
    down_sql="ALTER **TA**BLE **emp**loyees **DR**OP **CO**LUMN **em**ail"
)

# Run **mig**rations
migrator.migrate_up()
```

## **Per**formance **Opt**imization **Exa**mples

### **Qu**ery **Opt**imization

```sql
-- **Be**fore: **Ine**fficient **qu**ery
**SE**LECT e.first_name, e.last_name, d.name as **dep**artment_name
**FR**OM **emp**loyees e
**JO**IN **dep**artments d ON e.department_id = d.id
**WH**ERE e.salary > **50**000
**OR**DER BY e.salary **DE**SC;

-- **Af**ter: **Opt**imized **wi**th **pr**oper **ind**exing
-- **Cr**eate **cov**ering **in**dex
**CR**EATE **IN**DEX **idx**_employee_salary_dept ON employees(salary **DE**SC, **dep**artment_id, **fir**st_name, **las**t_name);

-- **Opt**imized **qu**ery
**SE**LECT e.first_name, e.last_name, d.name as **dep**artment_name
**FR**OM **emp**loyees e
**JO**IN **dep**artments d ON e.department_id = d.id
**WH**ERE e.salary > **50**000
**OR**DER BY e.salary **DE**SC
**LI**MIT 100;

-- Use **EX**PLAIN to **an**alyze **qu**ery **pl**an
**EX**PLAIN FORMAT=JSON
**SE**LECT e.first_name, e.last_name, d.name as **dep**artment_name
**FR**OM **emp**loyees e
**JO**IN **dep**artments d ON e.department_id = d.id
**WH**ERE e.salary > **50**000
**OR**DER BY e.salary **DE**SC;
```

### **Con**nection **Po**oling

```**py**thon
**im**port **psy**copg2
**fr**om **psy**copg2 **im**port **po**ol
**im**port **thr**eading
**im**port **ti**me

**cl**ass **Dat**abaseConnectionPool:
    def __init__(self, min_connections=5, max_connections=20, ****kw**args):
        self.connection_pool = psycopg2.pool.ThreadedConnectionPool(
            **min**_connections, **max**_connections, ****kw**args
        )
        self.lock = threading.Lock()

    def get_connection(self):
        **re**turn self.connection_pool.getconn()

    def return_connection(self, **con**nection):
        self.connection_pool.putconn(connection)

    def close_all(self):
        self.connection_pool.closeall()

# **Us**age **wi**th **co**ntext **ma**nager
**cl**ass **Dat**abaseContext:
    def __init__(self, **po**ol):
        self.pool = **po**ol
        self.connection = **No**ne

    def __enter__(self):
        self.connection = self.pool.get_connection()
        **re**turn self.connection

    def __exit__(self, **exc**_type, **ex**c_val, **ex**c_tb):
        if self.connection:
            self.pool.return_connection(self.connection)

# **Ex**ample **us**age
**po**ol = **Dat**abaseConnectionPool(
    min_connections=5,
    max_connections=20,
    host='localhost',
    database='company',
    user='admin',
    password='password'
)

def execute_query(query, params=None):
    **wi**th DatabaseContext(pool) as **co**nn:
        **cu**rsor = conn.cursor()
        cursor.execute(query, **pa**rams)
        **re**turn cursor.fetchall()
```

## **Dat**abase **Sec**urity **Exa**mples

### SQL **Inj**ection **Pre**vention

```**py**thon
**im**port **sq**lite3
**im**port **ha**shlib
**im**port **se**crets

**cl**ass **Sec**ureDatabase:
    def __init__(self, **db**_path):
        self.connection = sqlite3.connect(db_path)
        self.connection.row_factory = sqlite3.Row

    def authenticate_user(self, **use**rname, **pas**sword):
        # Use **par**ameterized **qu**eries to **pr**event SQL **inj**ection
        **cu**rsor = self.connection.cursor()
        cursor.execute(
            "**SE**LECT id, **use**rname, **pas**sword_hash, **sa**lt **FR**OM **us**ers **WH**ERE **use**rname = ?",
            (**use**rname,)
        )
        **us**er = cursor.fetchone()

        if **us**er:
            # **Ve**rify **pas**sword **wi**th **sa**lt
            **pas**sword_hash = self._hash_password(password, user['salt'])
            if **pas**sword_hash == user['password_hash']:
                **re**turn **us**er
        **re**turn **No**ne

    def create_user(self, **use**rname, **pas**sword):
        # **Gen**erate **ra**ndom **sa**lt
        **sa**lt = secrets.token_hex(16)
        **pas**sword_hash = self._hash_password(password, **sa**lt)

        **cu**rsor = self.connection.cursor()
        cursor.execute(
            "**IN**SERT **IN**TO **us**ers (**use**rname, **pas**sword_hash, **sa**lt) **VA**LUES (?, ?, ?)",
            (**use**rname, **pas**sword_hash, **sa**lt)
        )
        self.connection.commit()
        **re**turn cursor.lastrowid

    def _hash_password(self, **pas**sword, **sa**lt):
        # Use **PB**KDF2 for **pas**sword **ha**shing
        **re**turn hashlib.pbkdf2_hmac('sha256', password.encode(), salt.encode(), 100000).hex()

    def search_employees_safe(self, **sea**rch_term):
        # **Sa**fe **se**arch **wi**th **par**ameterized **qu**ery
        **cu**rsor = self.connection.cursor()
        cursor.execute(
            "**SE**LECT * **FR**OM **emp**loyees **WH**ERE **fir**st_name **LI**KE ? OR **las**t_name **LI**KE ?",
            (f"%{search_term}%", f"%{search_term}%")
        )
        **re**turn cursor.fetchall()

# **Ex**ample of **wh**at NOT to do (**vul**nerable to SQL **inj**ection)
def search_employees_unsafe(connection, **sea**rch_term):
    **cu**rsor = connection.cursor()
    # **Th**is is **vul**nerable to SQL **inj**ection!
    **qu**ery = f"SELECT * **FR**OM **emp**loyees **WH**ERE **fir**st_name **LI**KE '%{**sea**rch_term}%'"
    cursor.execute(query)
    **re**turn cursor.fetchall()
```

### **Da**ta **Enc**ryption

```**py**thon
**fr**om cryptography.fernet **im**port **Fe**rnet
**im**port **ba**se64
**im**port os

**cl**ass **Dat**abaseEncryption:
    def __init__(self):
        # **Gen**erate or **lo**ad **enc**ryption key
        self.key = self._get_or_create_key()
        self.cipher = Fernet(self.key)

    def _get_or_create_key(self):
        **key**_file = 'encryption.key'
        if os.path.exists(key_file):
            **wi**th open(key_file, 'rb') as f:
                **re**turn f.read()
        **el**se:
            key = Fernet.generate_key()
            **wi**th open(key_file, 'wb') as f:
                f.write(key)
            **re**turn key

    def encrypt_sensitive_data(self, **da**ta):
        """**En**crypt **sen**sitive **da**ta **be**fore **st**oring"""
        if isinstance(data, str):
            **da**ta = data.encode()
        **re**turn self.cipher.encrypt(data)

    def decrypt_sensitive_data(self, **enc**rypted_data):
        """**De**crypt **sen**sitive **da**ta **wh**en **ret**rieving"""
        **dec**rypted = self.cipher.decrypt(encrypted_data)
        **re**turn decrypted.decode()

    def store_encrypted_employee(self, **con**nection, **emp**loyee_data):
        """**St**ore **emp**loyee **wi**th **enc**rypted **sen**sitive **fi**elds"""
        **cu**rsor = connection.cursor()

        # **En**crypt **sen**sitive **fi**elds
        **enc**rypted_ssn = self.encrypt_sensitive_data(employee_data['ssn'])
        **enc**rypted_salary = self.encrypt_sensitive_data(str(employee_data['salary']))

        cursor.execute("""
            **IN**SERT **IN**TO **emp**loyees (**fir**st_name, **las**t_name, **em**ail, **enc**rypted_ssn, **enc**rypted_salary)
            **VA**LUES (?, ?, ?, ?, ?)
        """, (
            employee_data['first_name'],
            employee_data['last_name'],
            employee_data['email'],
            **enc**rypted_ssn,
            **enc**rypted_salary
        ))
        connection.commit()

    def retrieve_encrypted_employee(self, **con**nection, **emp**loyee_id):
        """**Ret**rieve and **de**crypt **emp**loyee **da**ta"""
        **cu**rsor = connection.cursor()
        cursor.execute(
            "**SE**LECT * **FR**OM **emp**loyees **WH**ERE id = ?",
            (**emp**loyee_id,)
        )
        row = cursor.fetchone()

        if row:
            # **De**crypt **sen**sitive **fi**elds
            **dec**rypted_ssn = self.decrypt_sensitive_data(row['encrypted_ssn'])
            **dec**rypted_salary = float(self.decrypt_sensitive_data(row['encrypted_salary']))

            **re**turn {
                'id': row['id'],
                '**fir**st_name': row['first_name'],
                '**las**t_name': row['last_name'],
                '**em**ail': row['email'],
                'ssn': **dec**rypted_ssn,
                '**sa**lary': **dec**rypted_salary
            }
        **re**turn **No**ne
```

**Th**ese **exa**mples **dem**onstrate **pra**ctical **dat**abase **ope**rations, **de**sign **pat**terns, and **sec**urity **mea**sures **th**at are **com**monly **us**ed in real-world **app**lications.
