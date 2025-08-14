import { Star } from "lucide-react";
import { Card, CardContent } from "@/components/ui/card";
import testimonial1 from "../assets/testimonial-1.webp";
import testimonial2 from "../assets/testimonial-2.webp";
import testimonial3 from "../assets/testimonial-3.webp";

const testimonials = [
  {
    name: "Sarah Chen",
    role: "Project Manager",
    avatar: testimonial1,
    content: "TaskTonic has completely transformed how I manage my projects. The interface is intuitive and the collaboration features are outstanding.",
    rating: 5
  },
  {
    name: "Marcus Rodriguez",
    role: "Freelance Designer",
    avatar: testimonial2,
    content: "I've tried dozens of to-do apps, but TaskTonic is the only one that actually helps me stay organized without feeling overwhelming.",
    rating: 5
  },
  {
    name: "Jessica Thompson",
    role: "Small Business Owner",
    avatar: testimonial3,
    content: "The smart reminders and team collaboration features have made our workflow so much smoother. Highly recommend!",
    rating: 5
  }
];

const Testimonials = () => {
  return (
    <section id="testimonials" className="py-20 bg-muted/50">
      <div className="container mx-auto px-4">
        <div className="text-center mb-16 animate-fade-in">
          <h2 className="text-3xl md:text-4xl lg:text-5xl font-bold text-foreground mb-6">
            Loved by{" "}
            <span className="bg-text-gradient bg-clip-text text-transparent">
              Thousands
            </span>
          </h2>
          <p className="text-xl text-muted-foreground max-w-3xl mx-auto mb-8">
            See what our users are saying about TaskTonic
          </p>
          
          <div className="flex items-center justify-center space-x-2 mb-4">
            {[...Array(5)].map((_, i) => (
              <Star key={i} className="w-6 h-6 fill-yellow-400 text-yellow-400" />
            ))}
            <span className="text-lg font-semibold text-foreground ml-2">4.9/5</span>
          </div>
          <p className="text-muted-foreground">
            Based on 2,847 reviews
          </p>
        </div>
        
        <div className="grid md:grid-cols-3 gap-8 max-w-6xl mx-auto">
          {testimonials.map((testimonial, index) => (
            <Card 
              key={index}
              className="border-0 shadow-card hover:shadow-lg transition-all duration-300 animate-fade-in"
              style={{ animationDelay: `${index * 0.2}s` }}
            >
              <CardContent className="p-8">
                <div className="flex items-center mb-4">
                  {[...Array(testimonial.rating)].map((_, i) => (
                    <Star key={i} className="w-4 h-4 fill-yellow-400 text-yellow-400" />
                  ))}
                </div>
                
                <p className="text-muted-foreground leading-relaxed mb-6">
                  "{testimonial.content}"
                </p>
                
                <div className="flex items-center">
                  <img 
                    src={testimonial.avatar}
                    alt={testimonial.name}
                    className="w-12 h-12 rounded-full object-cover mr-4"
                  />
                  <div>
                    <h4 className="font-semibold text-foreground">
                      {testimonial.name}
                    </h4>
                    <p className="text-sm text-muted-foreground">
                      {testimonial.role}
                    </p>
                  </div>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    </section>
  );
};

export default Testimonials;