$fn=16;

module axis(label) {
	scale(1 / 16) {
		// arrow
		rotate([0, 90, 0])
		rotate_extrude()
		polygon([ [0, 0], [0, 16], [2, 12], [1, 12], [1, 0] ]);
		
		// label
		translate([16, 0, 0])
		linear_extrude(1, center=true)
		text(label, 4, valign="center");
	}
}

rotate([ 0,   0,   0]) axis("X");
rotate([90,   0,  90]) axis("Y");
rotate([ 0, -90, -90]) axis("Z");
