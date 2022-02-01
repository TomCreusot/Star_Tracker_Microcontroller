% This code is sample code of the quest algorithm.
% This code was found and modified from:
% https://github.com/risherlock/Wahba/blob/master/matlab/algorithms/quest1981.m

clc % Clears terminal

% Creates a set of random unit points.
v_b = normc(rand(10, 3)');

% The expected output.
rotation = [1, 0, 0, pi/2];

% V_i is a rotated version of v_b.
v_i = v_b;

% Creates the weights.
w = zeros(numel(v_i)/3, 1);

% Randomly generates the weights.
% Rotates v_i by a slightly randomized version of rotation.
for i = 1:numel(v_b)/3
    angle = rotation(4) + (randn() / 4);
    axis = [rotation(1), rotation(2), rotation(3)];
    axis(1) = axis(1) + randn() / 5;
    axis(2) = axis(2) + randn() / 5;
    axis(3) = axis(3) + randn() / 5;
    axis = normr(axis);
    ax = [axis, angle];
    v_i(:,i) = axang2rotm(ax) * v_b(:,i);

    w(i) = rand();
end


tolerance = 10e-6;

B = (v_b.*repmat(w,[1,3])')*v_i';
Z = [B(2,3)-B(3,2); B(3,1)-B(1,3); B(1,2)-B(2,1)];
S = B + B';
sigma = trace(B);

delta = det(S);
kappa = trace(delta*inv(S));
kappa_1 = trace(adjoint(S));

a = sigma^2 - kappa;
b = sigma^2 + Z'*Z;
c = delta + Z'*S*Z;
d = Z'*S^2*Z;
constant = a*b + c*sigma - d;

% Characteristic equation for Newton-Raphson method: (Eqn 70) 
%   f(lambda) = lambda^4 - (a + b)*lambda^2 - c*lambda + constant = 0
%   where, constant = a*b + c*sigma - d
lambda = sum(w);
i = 0;
disp(["Lambda initial: ", lambda]);
last_lambda = 0.0;
while abs(lambda - last_lambda) >= tolerance
    last_lambda = lambda;
    
    f = lambda^4 - (a + b)*lambda^2 - c*lambda + constant;
    f_dot = 4*lambda^3 - 2*(a + b)*lambda - c;
    lambda = lambda - f/f_dot;
    i = i + 1;
end
disp(["Lambda final:", lambda]);
disp(["Iterations: ", i]);
  
% Eqn 66
omega = lambda;
alpha = omega^2 - sigma^2 + kappa;
beta  = omega - sigma;
gamma = (omega + sigma)*alpha - delta;

% Determine optimal quaternion
X = (alpha*eye(3) + beta*S + S^2)*Z; % Eqn 68
q_opt = [X; gamma]./sqrt(gamma^2 + norm(X)^2); % Eqn 69
%C_opt = quaternion2DCM(q_opt);

% Use matlab quaternion.
q_mat = [q_opt(4), q_opt(1), q_opt(2), q_opt(3)];

q_diff = conj(q_mat') * axang2quat(rotation);
angle = 2 * atan2(norm(q_diff(2:4)),q_diff(1));

angle_axis = quat2axang(q_mat);
disp(["Angle Axis output: ", [angle_axis(1), angle_axis(2), angle_axis(3)], " ", angle_axis(4) * 180 / pi]);



