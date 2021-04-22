from fastfood.models import Menuentries, Restaurants
from rest_framework import serializers


class MenuEntrySerializer(serializers.ModelSerializer):
    class Meta:
        model = Menuentries
        fields=['food_id', 'restaurant', 'creator', 'name', 'description', 'price', 'calories', 'creation_date', 'update_date']

class RestaurantSerializer(serializers.ModelSerializer):
    class Meta:
        model = Restaurants
        fields = ['restaurant_id', 'name', 'google_location', 'creator']