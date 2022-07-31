fn calc_collision(object1 : &mut Object, object2 : &mut Object) {
    // check if the object is colliding with itself
    let mut distance = object2.pos - object1.pos;
    let max_radius = object2.radius + object1.radius;
    if distance.x.abs() < max_radius && distance.y.abs() < max_radius && distance.z.abs() < max_radius  {
        let mut distance_length = distance.magnitude2();
        if distance_length < max_radius * max_radius {
            distance_length = distance_length.sqrt();
            // incase the object is directly at the other objects position, extrude it up
            if distance_length == 0.0 {
                distance_length = 0.000001;
                distance = Vector3::new(0.0, 0.000001, 0.0);
            }
            // new position calculation
            let distance_normalized = distance / distance_length;
            let distance_length_diff = max_radius - distance_length;
            let distance_length_diff_normalized = distance_normalized * distance_length_diff * 0.5;

            let split = object1.mass + object2.mass;
            object1.pos -= distance_length_diff_normalized * (object2.mass / split);
            object2.pos += distance_length_diff_normalized * (object1.mass / split);

            // apply friction
            //if object1.material_type == Material::SAND {
            //    object1.pos = object1.pos - object1.get_velocity() * 0.01 * inv_phys;
            //}
            //object2.pos = object2.pos - object2.get_velocity() * 0.01 * inv_phys;
        }
    }
}

fn calc_all_physics(objects : &mut Vec<Object>) {
    // loop through all objects
    let objects_len = objects.len();
    for object_index in 0 .. objects_len {
    // loop through all objects and see if they are colliding with this one
        for collision_object_index in 0 .. objects_len {
            if object_index == collision_object_index {continue}

            // get both mutable objects by splitting the mutable
            // if the object index is over the collision object index we must split it at the object index
            // if the collision object has a greater index we must split it there
            if object_index > collision_object_index {
                let (head, tail) = objects.split_at_mut(object_index);
                let object = &mut tail[0];
                let collision_object = &mut head[collision_object_index];

                calc_collision(object, collision_object);
            } else {
                let (head, tail) = objects.split_at_mut(collision_object_index);
                let object = &mut head[object_index];
                let collision_object = &mut tail[0];

                calc_collision(object, collision_object);
            }
        }
    }
}