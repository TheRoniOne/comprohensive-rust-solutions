pub struct User {
    name: String,
    age: u32,
    height: f32,
    visit_count: usize,
    last_blood_pressure: Option<(u32, u32)>,
}

pub struct Measurements {
    height: f32,
    blood_pressure: (u32, u32),
}

pub struct HealthReport<'a> {
    patient_name: &'a str,
    visit_count: u32,
    height_change: f32,
    blood_pressure_change: Option<(i32, i32)>,
}

impl User {
    pub fn new(name: String, age: u32, height: f32) -> Self {
        Self {
            name,
            age,
            height,
            visit_count: 0,
            last_blood_pressure: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn doctor_visits(&self) -> u32 {
        self.visit_count as u32
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    pub fn set_height(&mut self, new_height: f32) {
        self.height = new_height;
    }

    pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
        let original_height = self.height;
        let original_blood_pressure = self.last_blood_pressure;

        let blood_pressure_change = match original_blood_pressure {
            Some((original_bp1, original_bp2)) => {
                let (new_bp1, new_bp2) = measurements.blood_pressure;
                let final_bp1 = new_bp1 as i32 - original_bp1 as i32;
                let final_bp2 = new_bp2 as i32 - original_bp2 as i32;

                Some((final_bp1, final_bp2))
            }
            None => None,
        };

        self.visit_count += 1;
        self.set_height(measurements.height);
        self.last_blood_pressure = Some(measurements.blood_pressure);

        HealthReport {
            patient_name: self.name(),
            visit_count: self.doctor_visits(),
            height_change: original_height - self.height(),
            blood_pressure_change,
        }
    }
}

#[test]
fn test_behavior() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    println!("I'm {} and my age is {}", bob.name(), bob.age());
}

#[test]
fn test_height() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.height(), 155.2);
}

#[test]
fn test_set_age() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.age(), 32);
    bob.set_age(33);
    assert_eq!(bob.age(), 33);
}

#[test]
fn test_visit() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.doctor_visits(), 0);
    let report = bob.visit_doctor(Measurements {
        height: 156.1,
        blood_pressure: (120, 80),
    });
    assert_eq!(report.patient_name, "Bob");
    assert_eq!(report.visit_count, 1);
    assert_eq!(report.blood_pressure_change, None);

    let report = bob.visit_doctor(Measurements {
        height: 156.1,
        blood_pressure: (115, 76),
    });

    assert_eq!(report.visit_count, 2);
    assert_eq!(report.blood_pressure_change, Some((-5, -4)));
}
